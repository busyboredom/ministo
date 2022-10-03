mod common;

use std::env;

use fantoccini::{elements::Element, Locator};

use common::{initialize, wait, WebdriverClient};

#[tokio::main]
#[test]
async fn main() {
    let servers = initialize().await;
    let client = servers.web_driver;

    // Check that we're on the setup screen.
    client
        .wait_for_text(
            Locator::Id("nav-title"),
            "Welcome",
            "User not directed to the initial setup screen",
        )
        .await;

    let next = client.find(Locator::Id("next-setting")).await.unwrap();
    let back = client.find(Locator::Id("back-setting")).await.unwrap();
    let done = client.find(Locator::Id("done-setup")).await.unwrap();
    assert!(!next.is_enabled().await.unwrap());
    assert!(!back.is_enabled().await.unwrap());
    assert!(!done.is_displayed().await.unwrap());

    test_enter_address(&client, &next, &back, &done).await;
    test_enter_dir(&client, &next, &back, &done).await;
}

async fn test_enter_address(
    client: &WebdriverClient,
    next: &Element,
    back: &Element,
    done: &Element,
) {
    // Enter primary address.
    let textbox = client
        .find(Locator::Id("setup-monero-address"))
        .await
        .unwrap();
    textbox.send_keys("4A1WSBQdCbUCqt3DaGfmqVFchXScF43M6c5r4B6JXT3dUwuALncU9XTEnRPmUMcB3c16kVP9Y7thFLCJ5BaMW3UmSy93w3w").await.unwrap();

    assert!(!back.is_enabled().await.unwrap());
    assert!(!done.is_displayed().await.unwrap());
    client
        .wait_for_enabled(
            Locator::Id("next-setting"),
            true,
            "Next button should be enabled",
        )
        .await;

    // Go to next step.
    next.click().await.unwrap();

    client
        .wait_for_displayed(
            Locator::Id("next-setting"),
            false,
            "Next button should not be enabled",
        )
        .await;
    client
        .wait_for_enabled(
            Locator::Id("back-setting"),
            true,
            "Back button should be enabled",
        )
        .await;
    client
        .wait_for_displayed(
            Locator::Id("done-setup"),
            true,
            "Done button should be displayed",
        )
        .await;
    assert!(done.is_enabled().await.unwrap());
}

async fn test_enter_dir(client: &WebdriverClient, next: &Element, back: &Element, done: &Element) {
    assert!(!next.is_displayed().await.unwrap());
    assert!(back.is_enabled().await.unwrap());
    assert!(done.is_displayed().await.unwrap());

    let textbox = client
        .find(Locator::Id("setup-blockchain-dir"))
        .await
        .unwrap();
    wait(
        None,
        None,
        || async { textbox.text().await.unwrap().is_empty() },
        true,
        "There should be a default blockchain dir",
    )
    .await;

    // Enter directory location.
    let mut dir = env::temp_dir();
    dir.push("bitmonero");
    textbox.send_keys(dir.to_str().unwrap()).await.unwrap();

    assert!(!next.is_displayed().await.unwrap());
    assert!(back.is_enabled().await.unwrap());
    assert!(done.is_displayed().await.unwrap());

    // Finish setup.
    done.click().await.unwrap();

    // Check that we're on the home screen.
    client
        .wait_for_text(
            Locator::Id("nav-title"),
            "Home",
            "User not directed to the home screen",
        )
        .await;
    assert!(!next.is_displayed().await.unwrap());
    assert!(!back.is_displayed().await.unwrap());
    assert!(!done.is_displayed().await.unwrap());
}
