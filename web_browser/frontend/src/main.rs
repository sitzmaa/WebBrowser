use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window,HtmlInputElement};
use yew::function_component;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let url = use_state_eq(|| "https://www.example.com".to_string()); // State to store the web address
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());


    // Handle the text input change
    let on_input = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            url.set(input.value()); // Update the URL state
        })
    };

    let current_url = (*url).clone(); // Get the current URL


    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
    {
        let welcome = welcome.clone();
        use_effect_with(
            (*name).clone(), // Pass the actual value to observe changes
            move |name: &String| { // The closure accepts a reference to the dependency
                update_welcome_message(welcome, &*name);
                || ()
            },
        );
    }

    let message = (*welcome).clone();

    html! {
        <div>
            <h2 class={"heading"}>{message}</h2>
            <h2>{ "Enter a web address to render:" }</h2>
            <input type="text" value={current_url.clone()} oninput={on_input} placeholder="https://example.com" />
            <iframe src={current_url} width="800" height="600" title="Webpage Viewer" />
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: &str) {
    let name = name.to_string(); // Convert borrowed value to owned String

    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    });
}


#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    async fn hello(name: String) -> Result<JsValue, JsValue>;
}
