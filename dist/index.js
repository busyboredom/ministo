// Open sidenav.
document.getElementById("hamburger-menu").addEventListener("click", () => {
    document.getElementById("sideBar").style.width = "250px";
})

// Close sidenav.
document.getElementById("close-nav").addEventListener("click", () => {
    document.getElementById("sideBar").style.width = "0";
})

// Print status from backend.
document.getElementById("print-status").addEventListener("click", () => {
    document.getElementById("status").style.display = "block";
})

function updateStatus() {
    window.__TAURI__
        .invoke('print_status')
        .then((response) => {
            let summary = JSON.parse(response);
            document.getElementById("donation-percentage").innerText = summary.donate_level + " %";
            if (summary.hashrate.total[0] !== null) {
                document.getElementById("hashrate-10s").innerText = summary.hashrate.total[0].toFixed(0) + " H/s";
            }
            if (summary.hashrate.total[1] !== null) {
                document.getElementById("hashrate-60s").innerText = summary.hashrate.total[1].toFixed(0) + " H/s";
            }
            if (summary.hashrate.total[2] !== null) {
                document.getElementById("hashrate-15m").innerText = summary.hashrate.total[2].toFixed(0) + " H/s";
            }
        })
}
setInterval(updateStatus, 10000)
