var initHomeInterval = setInterval(function () {
    if (window.state.pagesLoaded) {

        // Go home if setup is complete, or welcome otherwise.
        var initConfigInterval = setInterval(function () {
            if (window.state.config != null) {
                if (setupComplete()) {
                    document.getElementById("home-container").style.display = "block";
                } else {
                    window.state.setupStep = 0;
                    document.getElementById("nav-title").innerText = "Welcome";
                    document.getElementById("welcome-container").style.display = "block";
                }

                clearTimeout(initConfigInterval);
            }
        }, 100);

        // LISTENERS ----------------------------------------------------------

        // Start mining.
        document.getElementById("start-mining").addEventListener("click", () => {
            window.__TAURI__
                .invoke('start_mining');
        })

        // Pause mining.
        document.getElementById("pause-mining").addEventListener("click", () => {
            window.__TAURI__
                .invoke('pause_mining');
        })

        // Resume mining.
        document.getElementById("resume-mining").addEventListener("click", () => {
            window.__TAURI__
                .invoke('resume_mining');
        })

        clearTimeout(initHomeInterval);
    }
}, 100);

// FUNCTIONS ----------------------------------------------------------

function startMining() {
    window.__TAURI__
        .invoke('start_mining');
}

function pauseMining() {
    window.__TAURI__
        .invoke('pause_mining');
}

function resumeMining() {
    window.__TAURI__
        .invoke('resume_mining');
}

function updateStatus(status) {
    let summary = JSON.parse(status);
    console.log(summary)

    // Display hashrate.
    if (summary.hashrate.total[0] !== null) {
        document.getElementById("hashrate-10s").innerText = summary.hashrate.total[0].toFixed(0) + " H/s";
    } else {
        document.getElementById("hashrate-10s").innerText = "0 H/s"
    }
}

function setupComplete() {
    let config = window.state.config;
    if (config.pool) {
        if (config.pool.Local) {
            if (config.pool.Local.monero_address) {
                return true
            }
        } else if (config.pool.Remote) {
            return true
        }
    }
    return false
}

// EVENTS -------------------------------------------------------------

window.__TAURI__.event.listen('xmrig-status', (event) => {
    updateStatus(event.payload);
})
