var initWelcomeInterval = setInterval(function () {
    if (window.state.pagesLoaded) {

        document.getElementById("setup-blockchain-dir").value = window.state.config.pool.Local.blockchain_dir;

        // LISTENERS ----------------------------------------------------------

        // Enable next or done.
        for (let input of document.getElementsByClassName("setup-input")) {
            input.addEventListener("keyup", () => {
                document.getElementById("next-setting").disabled = false;
            })
        }

        // Next setting.
        document.getElementById("next-setting").addEventListener("click", () => {
            document.getElementById("next-setting").disabled = true;
            let oldStep = "setup-step-" + window.state.setupStep;
            window.state.setupStep += 1;
            let newStep = "setup-step-" + window.state.setupStep;
            document.getElementById(oldStep).style.display = "none";
            document.getElementById(newStep).style.display = "block";
            document.getElementById("back-setting").disabled = false;
            if (newStep == "setup-step-1") {
                document.getElementById("next-setting").style.display = "none";
                document.getElementById("done-setup").style.display = "inline-block";
            }
        })

        // Back setting.
        document.getElementById("back-setting").addEventListener("click", () => {
            let oldStep = "setup-step-" + window.state.setupStep;
            window.state.setupStep -= 1;
            let newStep = "setup-step-" + window.state.setupStep;
            document.getElementById(oldStep).style.display = "none";
            document.getElementById(newStep).style.display = "block";
            if (window.state.setupStep == 0) {
                document.getElementById("back-setting").disabled = true;
            }
            document.getElementById("next-setting").style.display = "inline-block";
            document.getElementById("next-setting").disabled = false;
            document.getElementById("done-setup").style.display = "none";
        })

        // Done setup.
        document.getElementById("done-setup").addEventListener("click", () => {
            let moneroAddress = document.getElementById("setup-monero-address").value;
            let blockchainFolder = document.getElementById("setup-blockchain-dir").value;
            // Save settings.
            window.__TAURI__
                .invoke('save_settings', { address: moneroAddress, folder: blockchainFolder });
            // Reload config.
            window.__TAURI__.invoke('get_config')
                .then(config => window.state.config = config)
                .then(_ => window.displaySettings());
            // Go home.
            document.getElementById("welcome-container").style.display = "none";
            document.getElementById("home-container").style.display = "block";
            document.getElementById("nav-title").innerText = "Home";
        })

        // Help collapsibles.
        var collapsibles = document.getElementsByClassName("collapsible-help")
        for (let collapsible of collapsibles) {
            collapsible.addEventListener("click", function () {
                this.classList.toggle("active")
                var content = this.nextElementSibling
                if (content.style.display === "block") {
                    content.style.display = "none"
                } else {
                    content.style.display = "block"
                }
            })
        }

        clearTimeout(initWelcomeInterval);
    }
}, 100);
