use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn Term() -> impl IntoView {
    let input_ref: NodeRef<Input> = create_node_ref();
    let (output, set_output) =
        create_signal(vec![String::from("Hello world")]);
    let (input, set_input) =
        create_signal(String::from(""));

    view! {
        <div id="term">
            <div>
                <form on:submit=move |ev: SubmitEvent| {
                    ev.prevent_default();
                    set_output.update(|v| v.push(input_ref().unwrap().value()));
                    set_input("".into())
                }>
                    <label for="input">~></label>
                    <input type="text" id="input" node_ref=input_ref prop:value=input />
                </form>
            </div>
            { move || output.get()
                .into_iter()
                .rev()
                .map(|s| view! { <code> {s} </code> })
                .collect::<Vec<_>>()
            }
        </div>
    }
}
