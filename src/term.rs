use leptos::ev::KeyboardEvent;
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
    let (form, set_form) = create_signal(String::new());
    let (term_output, term_output_write) =
        create_signal(vec![Row::from("testline")]);
    let input: NodeRef<Input> = create_node_ref();
    let keydown = move |ev: KeyboardEvent| {
        ev.prevent_default();
        term_output_write.update(move |lines| match &*ev.key() {
            "Backspace" => {
                set_form.update(|s| {
                    let mut t = s.chars();
                    t.next_back();
                    *s = t.as_str().into()
                });
            }
            "Enter" => {
                lines.push(input().unwrap().value().into());
                set_form.set(String::new());
            }
            _ => set_form.update(move |s| s.push(ev.key().chars().next().unwrap())),
        });
    };
    let size = || match window().inner_width().unwrap().as_f64().unwrap() as i32 {
        0..=700 => 50,
        _ => 80,
    };
    create_effect(move |_| set_form.set("hello world".into()));
    view! {
        <div id="term">
            <ul>
                <For
                    each=term_output
                    key=|line| line.0
                    children=move |line| {
                        view! { <code>{line.1}</code> }
                    }
                />
            </ul>
            <form>
                <label>"cmd ~> "</label>
                <input
                    type="text"
                    node_ref=input
                    on:keydown=keydown
                    prop:value=form
                    prop:size=size
                />

            </form>
        </div>
    }
}
