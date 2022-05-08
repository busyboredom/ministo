var initSettingsInterval = setInterval(function () {
    if (window.state.pagesLoaded) {
        window.displaySettings();

        // LISTENERS --------------------------------------------------

        // Browse.
        document.getElementById("select-blockchain-folder").addEventListener("click", () => {
            window.__TAURI__
                .invoke('select_blockchain_folder');
            document.getElementById("save-settings").disabled = false;
        })

        // Enable saving.
        document.getElementById("monero-address").addEventListener("keyup", () => {
            document.getElementById("save-settings").disabled = false;
        })
        document.getElementById("blockchain-dir").addEventListener("keyup", () => {
            document.getElementById("save-settings").disabled = false;
        })

        // Save settings.
        document.getElementById("save-settings").addEventListener("click", () => {
            let moneroAddress = document.getElementById("monero-address").value;
            let blockchainFolder = document.getElementById("blockchain-dir").value;
            window.__TAURI__
                .invoke('save_settings', { address: moneroAddress, folder: blockchainFolder });

            document.getElementById("save-settings").disabled = true;
            document.getElementById("save-effect-notice").style.display = "block";
        })

        clearTimeout(initSettingsInterval);
    }
}, 100);

// FUNCTIONS ----------------------------------------------------------

window.displaySettings = function () {
    document.getElementById("monero-address").value = window.state.config.pool.Local.monero_address;
    document.getElementById("blockchain-dir").value = window.state.config.pool.Local.blockchain_dir;
}

// EVENTS -------------------------------------------------------------

window.__TAURI__.event.listen('blockchain-folder-selected', (event) => {
    document.getElementById("blockchain-dir").value = event.payload;
})