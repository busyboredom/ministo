const pages = ["home", "settings", "diagnostics"];

window.state = {
    xmrig: {
        running: false,
        paused: false,
    },
    activePage: "home",
    pagesLoaded: false,
    diagnostics: {
        currentTab: "xmrig"
    },
    config: null,
}

// Load pages.
for (let page of pages) {
    loadPage(page);
}
window.state.pagesLoaded = true;

// Open homepage.
navigate("home");

// Retrieve config.
window.__TAURI__.invoke('get_config')
    .then(config => window.state.config = config);

// LISTENERS ----------------------------------------------------------

// Open sidenav.
document.getElementById("hamburger-menu").addEventListener("click", () => {
    document.getElementById("sideBar").style.width = "250px";
})

// Close sidenav.
document.getElementById("close-nav").addEventListener("click", () => {
    document.getElementById("sideBar").style.width = "0";
})

// Go to home.
document.getElementById("home-nav").addEventListener("click", () => {
    navigate("home");
})
document.getElementById("ministo-icon").addEventListener("click", () => {
    navigate("home");
})

// Go to settings.
document.getElementById("settings-nav").addEventListener("click", () => {
    navigate("settings");
})

// Go to diagnostics.
document.getElementById("diagnostics-nav").addEventListener("click", () => {
    navigate("diagnostics");
})

// FUNCTIONS ----------------------------------------------------------

// Navigate to specified page.
function navigate(newPage) {
    for (let oldPage of pages) {
        document.getElementById(oldPage).style.display = "none";
    }

    let capsName = newPage.charAt(0).toUpperCase() + newPage.slice(1);
    document.getElementById("nav-title").innerText = capsName;
    document.getElementById("sideBar").style.width = "0";

    document.getElementById(newPage).style.display = "block";
    state.activePage = newPage;
}

function loadPage(page) {
    let contentArea = document.getElementById(page);
    fetch(page + "/" + page + ".html")
        .then(content => content.text())
        .then(text => contentArea.innerHTML = text);
}