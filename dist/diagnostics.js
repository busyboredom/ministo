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

    var stdoutContainer = document.getElementById("stdout-container");
    stdoutContainer.scrollTop = stdoutContainer.scrollHeight;
}

function updateStdout(tabName, event) {
    const stdout = document.getElementById(tabName + "-stdout");

    let newLine = document.createElement("p");
    newLine.innerHTML = event.payload;
    stdout.appendChild(newLine);

    if (stdout.childElementCount > 1000) {
        stdout.removeChild(stdout.firstElementChild);
    }

    var stdoutContainer = document.getElementById("stdout-container");
    stdoutContainer.scrollTop = stdoutContainer.scrollHeight;
}

// EVENTS -------------------------------------------------------------

window.__TAURI__.event.listen('xmrig-stdout', (event) => {
    updateStdout("xmrig", event);
})