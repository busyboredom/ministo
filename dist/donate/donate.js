var initDonateInterval = setInterval(function () {
    if (window.state.pagesLoaded) {

        // Generate QR codes.
        let ministoAddress = document.getElementById("ministo-address").innerText;
        let p2poolAddress = document.getElementById("p2pool-address").innerText;
        let xmrigAddress = document.getElementById("xmrig-address").innerText;
        document.getElementById("ministo-qrcode").innerHTML = qrCode(ministoAddress);
        document.getElementById("p2pool-qrcode").innerHTML = qrCode(p2poolAddress);
        document.getElementById("xmrig-qrcode").innerHTML = qrCode(xmrigAddress);

        // LISTENERS ----------------------------------------------------------

        // XMRig tab.
        document.getElementById("ministo-donate-tab").addEventListener("click", () => {
            newDonateTab("ministo");
        })

        // P2pool tab.
        document.getElementById("p2pool-donate-tab").addEventListener("click", () => {
            newDonateTab("p2pool");
        })

        // Monerod tab.
        document.getElementById("xmrig-donate-tab").addEventListener("click", () => {
            newDonateTab("xmrig");
        })

        // Copy ministo address.
        document.getElementById("ministo-address-copy").addEventListener("click", () => {
            let address = document.getElementById("ministo-address").innerText;
            window.__TAURI__.clipboard.writeText(address);
        })

        // Copy p2pool address.
        document.getElementById("p2pool-address-copy").addEventListener("click", () => {
            let address = document.getElementById("p2pool-address").innerText;
            window.__TAURI__.clipboard.writeText(address);
        })

        // Copy xmrig address.
        document.getElementById("xmrig-address-copy").addEventListener("click", () => {
            let address = document.getElementById("xmrig-address").innerText;
            window.__TAURI__.clipboard.writeText(address);
        })

        clearTimeout(initDonateInterval);
    }
}, 100);

// FUNCTIONS ----------------------------------------------------------

function newDonateTab(tabName) {
    let activeTab = document.getElementById("donate-tabs").getElementsByClassName("active");
    activeTab[0].className = activeTab[0].className.replace(" active", "");

    document.getElementById(tabName + "-donate-tab").className += " active";

    let donateTabs = document.getElementsByClassName("donate");
    for (let tab of donateTabs) {
        tab.style.display = "none";
    }

    document.getElementById(tabName + "-donate").style.display = "flex";
}

function qrCode(address) {
    const qr = qrcode(0, "M");
    qr.addData(address);
    qr.make();
    return qr.createSvgTag({ scalable: true });
}
