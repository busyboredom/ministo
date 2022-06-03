use std::env;

use anyhow::{Error, Result};
use log::{debug, error, info};

const ROOT_UID: u32 = 0;

#[derive(Debug)]
pub struct UserPermissions {
    /// Read uid of user who called sudo, or None if not run with sudo.
    sudo_uid: Option<Uid>,
    current_uid: Uid,
}

impl UserPermissions {
    pub fn new() -> Result<UserPermissions> {
        // Get uid of user who called sudo, if applicable.
        let sudo_uid = env::var("SUDO_UID")
            .ok()
            .and_then(|str| str.parse().ok())
            .and_then(|i| if i == ROOT_UID { None } else { Some(i) })
            .map(Uid);

        // Prepare pointers.
        let (mut ruid, mut euid, mut suid) = (0u32, 0u32, 0u32);
        let (ruid_ptr, euid_ptr, suid_ptr): (*mut u32, *mut u32, *mut u32) =
            (&mut ruid, &mut euid, &mut suid);

        // Read current uids.
        unsafe {
            if libc::getresuid(ruid_ptr, euid_ptr, suid_ptr) != 0 {
                return Err(Error::msg("Failed to read current uids."));
            };
        }

        Ok(UserPermissions {
            sudo_uid,
            current_uid: Uid(euid),
        })
    }

    /// Drops to non-root permissions if available.
    pub fn reduce_permissions(&mut self) -> Result<()> {
        info!("Dropping permissions.");
        if !self.is_root() {
            return Ok(());
        }

        match self.sudo_uid {
            Some(uid) => {
                self.write_euid(uid)?;
                self.write_ruid(uid)?;
            }
            None => {
                return Err(Error::msg(
                    "No SUDO_UID to drop to; did you forget to run with sudo?",
                ))
            }
        };

        // Set HOME env variable to home of user who called sudo.
        match UserPermissions::home_dir() {
            Ok(home) => {
                println!("{}", home);
                env::set_var("HOME", home);
            }
            Err(e) => error!("Failed to read SUDO_USER's home, using current home: {}", e),
        };
        Ok(())
    }

    /// Elevates to root permissions if allowed.
    pub fn elevate_permissions(&mut self) -> Result<()> {
        info!("Elevating permissions.");
        if self.is_root() {
            return Ok(());
        }

        self.write_euid(Uid(ROOT_UID))?;
        Ok(())
    }

    /// Returns true if running as root.
    pub fn is_root(&self) -> bool {
        self.current_uid == Uid(ROOT_UID)
    }

    /// Attempts to read the current uids, returning a tuple of the form (ruid, euid, suid).
    fn read_uids(&mut self) -> Result<(Uid, Uid, Uid)> {
        // Prepare pointers.
        let (mut ruid, mut euid, mut suid) = (0u32, 0u32, 0u32);
        let (ruid_ptr, euid_ptr, suid_ptr): (*mut u32, *mut u32, *mut u32) =
            (&mut ruid, &mut euid, &mut suid);

        // Read current uids.
        unsafe {
            if libc::getresuid(ruid_ptr, euid_ptr, suid_ptr) != 0 {
                return Err(Error::msg("Failed to read uids."));
            };
        }

        self.current_uid = Uid(euid);
        Ok((Uid(ruid), Uid(euid), Uid(suid)))
    }

    /// Attempts to set current Euid to the Uid provided, returning the old Euid if successful.
    ///
    /// Also sets suid to the old Euid, making it possible to return to the old Euid if desired.
    fn write_euid(&mut self, uid: Uid) -> Result<Uid> {
        debug!("Setting euid to: {}", uid.0);
        let (ruid, old_euid, suid) = self
            .read_uids()
            .map_err(|_| Error::msg("Failed to read current uids"))?;
        debug!(
            "User IDs before setting euid:\nruid: {}, euid: {}, suid: {}",
            ruid.0, old_euid.0, suid.0
        );

        unsafe {
            // Set euid and move old value to suid (so we can return to suid later if desired).
            if libc::setresuid(ruid.0, uid.0, old_euid.0) != 0 {
                error!("Failed to set euid.");
            };
        }

        let (ruid, new_euid, suid) = self
            .read_uids()
            .map_err(|_| Error::msg("Failed to read new uids"))?;
        debug!(
            "User IDs after setting euid:\nruid: {}, euid: {}, suid: {}",
            ruid.0, new_euid.0, suid.0
        );

        self.current_uid = new_euid;
        Ok(suid)
    }

    /// Attempts to set current Ruid to the Uid provided, returning the old Ruid if successful.
    fn write_ruid(&mut self, uid: Uid) -> Result<Uid> {
        debug!("Setting ruid to: {}", uid.0);
        let (old_ruid, euid, suid) = self
            .read_uids()
            .map_err(|_| Error::msg("Failed to read current uids"))?;
        debug!(
            "User IDs before setting ruid:\nruid: {}, euid: {}, suid: {}",
            old_ruid.0, euid.0, suid.0
        );

        unsafe {
            // Set euid and move old value to suid (so we can return to suid later if desired).
            if libc::setresuid(uid.0, euid.0, suid.0) != 0 {
                error!("Failed to set ruid.");
            };
        }

        let (new_ruid, euid, suid) = self
            .read_uids()
            .map_err(|_| Error::msg("Failed to read new uids"))?;
        debug!(
            "User IDs after setting ruid:\nruid: {}, euid: {}, suid: {}",
            new_ruid.0, euid.0, suid.0
        );

        self.current_uid = euid;
        Ok(old_ruid)
    }

    /// Attempts to read the home dir of the user who called sudo.
    fn home_dir() -> Result<String> {
        let sudo_user = env::var("SUDO_USER")?;

        let mut getpw_string_buf = [0; 4096];
        let mut passwd: libc::passwd = unsafe { std::mem::zeroed() };
        let mut passwd_out: *mut libc::passwd = std::ptr::null_mut();

        let username = std::ffi::CString::new(sudo_user)?;
        let result = unsafe {
            libc::getpwnam_r(
                username.as_ptr(),
                &mut passwd as *mut _,
                getpw_string_buf.as_mut_ptr(),
                getpw_string_buf.len() as libc::size_t,
                &mut passwd_out as *mut _,
            )
        };
        if result == 0 {
            unsafe {
                Ok(std::ffi::CStr::from_ptr(passwd.pw_dir)
                    .to_string_lossy()
                    .into_owned())
            }
        } else {
            Err(Error::from(std::io::Error::from_raw_os_error(result)))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Uid(u32);
