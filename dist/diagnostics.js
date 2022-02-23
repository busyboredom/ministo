// LISTENERS ----------------------------------------------------------

var initDiagnosticsInterval = setInterval(function () {
    if (window.state.pagesLoaded) {
        // XMRig tab.
        document.getElementById("xmrig-tab").addEventListener("click", () => {
            newTab("xmrig");
        })

        // P2pool tab.
        document.getElementById("p2pool-tab").addEventListener("click", () => {
            newTab("p2pool");
        })

        // Monerod tab.
        document.getElementById("monerod-tab").addEventListener("click", () => {
            newTab("monerod");
        })

        clearTimeout(initDiagnosticsInterval);
    }
}, 100);

// FUNCTIONS ----------------------------------------------------------

function newTab(tabName) {
    let activeTab = document.getElementsByClassName("active");
    activeTab[0].className = activeTab[0].className.replace(" active", "");

    document.getElementById(tabName + "-tab").className += " active";

    let terminals = document.getElementsByClassName("terminal");
    for (let term of terminals) {
        term.style.display = "none";
    }

    document.getElementById(tabName + "-stdout").style.display = "block";
    updateStdout(tabName);
}

function updateStdout(tabName) {
    const stdout = document.getElementById(tabName + "-stdout");
    let state = window.state[tabName];
    if (typeof state.stdout != "undefined") {
        stdout.innerHTML = state.stdout.join("");
    }

    var stdoutContainer = document.getElementById("stdout-container");
    stdoutContainer.scrollTop = stdoutContainer.scrollHeight;
}

// EVENTS -------------------------------------------------------------

window.__TAURI__.event.listen('xmrig-stdout', (event) => {
    if (typeof window.state.xmrig.stdout == "undefined") {
        window.state.xmrig.stdout = "";
    }
    window.state.xmrig.stdout.push(event.payload);

    if (window.state.xmrig.stdout.length > 1000) {
        window.state.xmrig.stdout.shift();
    }

    if (window.state.diagnostics.currentTab == "xmrig") {
        updateStdout("xmrig");
    }
})