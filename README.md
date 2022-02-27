# Ministo

## Configuration
Ministo's configuration file location is dependent on your operating system.
* Windows: `%AppData%\Local\ministo.json`
* Unix: `$HOME/.config/ministo.json`

The default config file location can be overwritten with a command-line flag:
```bash
ministo --config my/custom/config/folder
```

## Build and Run

### Build preperation
1. Clone the repo: `git clone https://github.com/VennilaPugazhenthi/ministo.git`
2. Build XMRig: https://xmrig.com/docs/miner/build
3. Rename xmrig binary according to your device's architecture and operating system, for example:
    * Windows: `xmrig-x86_64-pc-windows-msvc.exe`  
    * Linux: `xmrig-x86_64-unknown-linux-gnu`    
    * MacOS: `xmrig-x86_64-apple-darwin`
4. Place the renamed xmrig binary in `ministo` > `src-tauri` > `external-bin`
5. Build P2Pool: https://github.com/SChernykh/p2pool#build-instructions
6. Repeat steps 3 and 4 for p2pool


### How to run with hot reloading (for development only)
1. Install devserver with `cargo install devserver`
2. Run devserver with `devserver --path 'dist' --reload`
2. Start ministo with `cargo tauri dev`

### How to build release binary
1. Run `cargo tauri build`