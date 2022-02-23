setInterval(updateStatus, 10000)

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

function updateStatus() {
    window.__TAURI__
        .invoke('xmrig_status')
        .then((response) => {
            let summary = JSON.parse(response);

            // Display hashrate.
            if (summary.hashrate.total[0] !== null) {
                document.getElementById("hashrate-10s").innerText = summary.hashrate.total[0].toFixed(0) + " H/s";
            }
        });
}