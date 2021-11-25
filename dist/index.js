function openNav() {
    document.getElementById("sideBar").style.width = "250px";
}

function closeNav() {
    document.getElementById("sideBar").style.width = "0";
}

function printStatus() {
    window.__TAURI__
        .invoke('print_status')
        .then((response) => {
            document.getElementById("status").innerText = response;
        })
}