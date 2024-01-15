use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use uuid::Uuid;

#[derive(Clone)]
struct Row(Uuid, String);

impl<T> From<T> for Row
    where
        String: From<T>,
{
    fn from(value: T) -> Self {
        Self(Uuid::new_v4(), value.into())
    }
}

#[component]
pub fn Term() -> impl IntoView {
    fn input_size() -> u16 {
        match window().inner_width().unwrap().as_f64().unwrap() as u16 {
            0..=850 => 50,
            851..=1100 => 70,
            _ => 80,
        }
    }
    let ((input_r, input_w), input_ref) =
        (create_signal(String::new()), create_node_ref::<Input>());
    let term_output = create_rw_signal(vec![Row::from("hello world")]);
    let term_size = create_rw_signal(0);
    let handle = window_event_listener(ev::resize, move |_| term_size.set(input_size()));
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        term_output.update(|lines| lines.push(input_r.get().into()));
        input_w.set("".into());
        input_ref.get().unwrap().set_value("");
    };
    create_effect(move |_| term_size.set(input_size()));
    on_cleanup(move || handle.remove());
    view! {
        <div id="term">
            <ul>
                <For
                    each=move || { term_output.get().into_iter().rev() }
                    key=|line| line.0
                    children=move |line| {
                        view! { <code>{line.1}</code> }
                    }
                />

            </ul>
            <form on:submit=on_submit>
                <label>"cmd~> "</label>
                <input
                    type="text"
                    node_ref=input_ref
                    value=input_r
                    size=term_size
                    maxlength=term_size
                    on:input=move |ev| input_w.set(event_target_value(&ev))
                />
            </form>
        </div>
    }
}
