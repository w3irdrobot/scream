use dioxus::prelude::*;
use log::debug;
use nostr_sdk::prelude::*;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app)
}

async fn scream_into_the_void(content: &str) -> Result<String> {
    let keys = Keys::generate();
    let client = Client::new(&keys);

    #[cfg(not(target_arch = "wasm32"))]
    client
        .add_relay("wss://nostr.mutinywallet.com", None)
        .await?;
    #[cfg(target_arch = "wasm32")]
    client.add_relay("wss://nostr.mutinywallet.com").await?;

    client.connect().await;

    let id = client.publish_text_note(content, &[]).await?;

    Ok(id.to_bech32()?)
}

fn app(cx: Scope) -> Element {
    let content: &UseState<String> = use_state(cx, String::new);
    let scream = move |_| {
        to_owned![content];
        async move {
            let event_id = scream_into_the_void(&content.current())
                .await
                .expect("scream error");
            debug!("event_id: {}", event_id);
        }
    };

    cx.render(rsx! (
        div {
            id: "main",
            h1 {
                "Scream into the Void"
            }
            h2 {
                "What is this?"
            }
            p {
                "This was originally created as an experiment with Dioxus. I just wanted to see how difficult it actually was to make a multi-platform project using Dioxus and its tooling. However, as I thought about it, someone may find some catharsis by getting their feelings out and sending them to everyone and no one."
            }
            p {
                "Each message sent generates a new private/public keypair, connects to "
                a{
                    href: "https://github.com/MutinyWallet/blastr",
                    target: "_blank",
                    "blastr"
                }
                ", and sends the note."
            }
            textarea{
                oninput: |evt| content.set(evt.value.clone()),
                "{content}"
            }
            button{
                onmouseup: scream,
                "Submit"
            }
            p {
                "Check out the code "
                a {
                    href: "https://github.com/w3irdrobot/scream",
                    target: "_blank",
                    "on GitHub"
                }
                "!"
            }
        }
    ))
}
