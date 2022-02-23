const pages = ["home", "diagnostics"];

window.state = {
    xmrig: {
        running: false,
        paused: false,
        stdout: [],
    },
    p2pool: {
        stdout: [],
    },
    monerod: {
        stdout: [],
    },
    activePage: "home",
    pagesLoaded: false,
    diagnostics: {
        currentTab: "xmrig"
    }
}

// Load pages.
for (let page of pages) {
    loadPage(page);
}
window.state.pagesLoaded = true;

// Open homepage.
navigate("home");

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

    document.getElementById("sideBar").style.width = "0";

    document.getElementById(newPage).style.display = "block";
    state.activePage = newPage;
}

function loadPage(page) {
    let contentArea = document.getElementById(page);
    fetch(page + ".html")
        .then(content => content.text())
        .then(text => contentArea.innerHTML = text);
}