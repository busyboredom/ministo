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
    window.__TAURI__
        .invoke('print_status')
        .then((response) => {
            document.getElementById("status").innerText = response;
        })
})
