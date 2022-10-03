use std::{
    env,
    future::Future,
    process::{Child, Command},
    task::Context,
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::Result;
use fantoccini::{elements::Element, wd::Capabilities, Client, ClientBuilder, Locator};
use futures::task;
use log::debug;
use nix::{sys::signal, unistd::Pid};
use serde_json::json;
use tempfile::{Builder as TempfileBuilder, TempDir};
use tokio::time::{sleep, Instant};

pub async fn initialize() -> Servers {
    env_logger::init();
    Servers::init().await
}

pub fn new_temp_dir() -> TempDir {
    TempfileBuilder::new()
        .prefix("temp_db_")
        .rand_bytes(16)
        .tempdir()
        .expect("failed to generate temporary directory")
}

pub struct Servers {
    pub web_driver: WebdriverClient, // Declared first so that the async call on drop gets run to completion.
    pub dev: JoinHandle<()>,
    pub tauri_driver: CommandChild,
}

impl Servers {
    pub async fn init() -> Servers {
        let dev = thread::spawn(|| {
            debug!("Starting devserver");
            devserver_lib::run("localhost", 8080, "../dist", false, "");
        });

        debug!("Starting tauri-driver");
        let tauri_driver = CommandChild(
            Command::new("tauri-driver")
                .spawn()
                .expect("failed to start tauri-driver"),
        );

        debug!("Starting webdriver client");
        let mut ministo_path =
            env::var("CARGO_MANIFEST_DIR").expect("failed to read target directory path");
        ministo_path.push_str("/target/debug/ministo");
        ministo_path.push_str(env::consts::EXE_EXTENSION);
        let config_path = new_temp_dir();
        let config_path = config_path
            .path()
            .to_str()
            .expect("Failed to generate dep dir");
        let mut capabilities = Capabilities::new();
        capabilities.insert(
            "tauri:options".to_string(),
            json!({"application": ministo_path, "args": ["--config", config_path]}),
        );
        let web_driver = {
            WebdriverClient(
                ClientBuilder::native()
                    .capabilities(capabilities)
                    .connect("http://localhost:4444")
                    .await
                    .expect("failed to connect to WebDriver"),
            )
        };
        Servers {
            dev,
            tauri_driver,
            web_driver,
        }
    }
}

pub struct WebdriverClient(pub Client);

impl Drop for WebdriverClient {
    fn drop(&mut self) {
        let client = self.0.clone();
        let mut fut = Box::pin(client.close());
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            if fut.as_mut().poll(&mut cx).is_ready() {
                debug!("Webdriver client closed");
                break;
            }
        }
    }
}

impl WebdriverClient {
    pub async fn find(&self, search: Locator<'_>) -> Result<Element> {
        Ok(self.0.find(search).await?)
    }

    pub async fn wait_for_text(&self, locator: Locator<'_>, text: &str, message: &str) {
        wait(
            None,
            None,
            || async { self.find(locator).await.unwrap().text().await.unwrap() },
            text.to_string(),
            message,
        )
        .await;
    }

    pub async fn wait_for_enabled(&self, locator: Locator<'_>, enabled: bool, message: &str) {
        wait(
            None,
            None,
            || async {
                self.find(locator)
                    .await
                    .unwrap()
                    .is_enabled()
                    .await
                    .unwrap()
            },
            enabled,
            message,
        )
        .await;
    }

    pub async fn wait_for_displayed(&self, locator: Locator<'_>, displayed: bool, message: &str) {
        wait(
            None,
            None,
            || async {
                self.find(locator)
                    .await
                    .unwrap()
                    .is_displayed()
                    .await
                    .unwrap()
            },
            displayed,
            message,
        )
        .await;
    }
}

pub struct CommandChild(pub Child);

impl Drop for CommandChild {
    fn drop(&mut self) {
        debug!("Closing child process");
        // Kill using SIGTERM because SIGKILL is not reliably propagated to grandchildren.
        signal::kill(Pid::from_raw(self.0.id() as i32), signal::Signal::SIGTERM)
            .expect("cannot send SIGTERM");
        self.0.wait().unwrap();
        debug!("Child process closed");
    }
}

pub async fn wait<Fut, T>(
    max: Option<Duration>,
    period: Option<Duration>,
    actual: impl Fn() -> Fut,
    expected: T,
    message: &str,
) where
    Fut: Future<Output = T> + Send,
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    let max = max.unwrap_or(Duration::from_secs(10));
    let period = period.unwrap_or(Duration::from_millis(100));
    let mut res = None;
    let start = Instant::now();

    while start.elapsed() < max {
        res = Some(actual().await);
        if res.as_ref() == Some(&expected) {
            break;
        }
        sleep(period).await;
    }

    if let Some(act) = res {
        assert_eq!(act, expected, "{}", message);
    } else {
        panic!("{}", message);
    }
}
