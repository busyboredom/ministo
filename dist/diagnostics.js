// FUNCTIONS ----------------------------------------------------------

function updateStdout(event) {
    if (typeof window.state.xmrig.stdout == "undefined") {
        window.state.xmrig.stdout = "";
    }
    window.state.xmrig.stdout.push(event.payload);

    const stdout = document.getElementById("xmrig-stdout");
    if (typeof window.state.xmrig.stdout != "undefined") {
        stdout.innerHTML = window.state.xmrig.stdout.join("");
    }

    var stdoutContainer = document.getElementById("stdout-container");
    stdoutContainer.scrollTop = stdoutContainer.scrollHeight;

    if (window.state.xmrig.stdout.length > 1000) {
        window.state.xmrig.stdout.shift();
    }
}

// EVENTS -------------------------------------------------------------

window.__TAURI__.event.listen('xmrig-stdout', (event) => {
    updateStdout(event);
})