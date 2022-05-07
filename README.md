# Ministo

## Configuration
Ministo's configuration file location is dependent on your operating system.
* Windows: `%AppData%\Local\ministo.json`
* Linux: `$HOME/.config/ministo.json`

The default config file location can be overwritten with a command-line flag:
```bash
ministo --config my/custom/config/folder
```

## Build and Run

### Preperation
1. Clone the repo: `git clone --recurse-submodules
   https://github.com/VennilaPugazhenthi/ministo.git`
3. Run monerod as described here: https://p2pool.io/#help

### How to run with hot reloading (for development only)
1. Install devserver with `cargo install devserver`
2. Run the `dev` script:
    * Linux: `./script/linux/dev.sh`
    * Winidows: TODO

### How to build release binary
1. Run the `build` script:
    * Linux: `./script/linux/build.sh`
    * Winidows: TODO