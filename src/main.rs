use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let input_text = use_state(String::new);

    let onsubmit = {
        let input_text = input_text.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            input_text.set(String::new());
        }
    };

    let oninput = {
        let input_text = input_text.clone();
        move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            input_text.set(value);
        }
    };

    html! {
        <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh; margin: 0;">
            <form {onsubmit}>
                <input
                    type="text"
                    value={(*input_text).clone()}
                    {oninput}
                    placeholder="Enter something"
                    style="font-size: 1.5rem; padding: 0.5rem; margin-bottom: 2rem; width: 300px;"
                />
                <button
                    type="submit"
                    style="font-size: 1.5rem; padding: 0.5rem 1rem;"
                >
                    { "Clear" }
                </button>
            </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
