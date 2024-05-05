use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use std::collections::HashMap;
use uuid::Uuid;

#[component]
pub fn Term() -> impl IntoView {
    let input_ref: NodeRef<Input> = create_node_ref();
    let output = create_rw_signal(vec![Line::from("Hello world")]);
    let input = create_rw_signal(String::from(""));

    let command_eval = move || {
        let elem = input_ref.get().unwrap();
        let value = elem.value();
        let mut tokens = value.split_whitespace();

        let (command, args) = (tokens.next(), tokens.collect::<Vec<_>>());

        output.update(|history| {
            if let Some(command) = command {
                match command {
                    "echo" => (),
                    _ => history.push(format!("{command}, {:?}", args).into()),
                };
            }
        });
    };

    view! {
        <div id="term">
            <div>
                <form on:submit=move |ev: SubmitEvent| {
                    ev.prevent_default();
                    command_eval();
                    input.set("".into())
                }>
                    <label for="input">{"~>"}</label>
                    <input type="text" node_ref=input_ref prop:value=input />
                </form>
            </div>
            <div class="inner">
                <For
                    each=move || output.get().into_iter().rev()
                    key=|line| line.id()
                    children=move |line| view! { <code> {line} </code> }
                />
            </div>
        </div>
    }
}

#[derive(Clone)]
struct Line(Uuid, String);
impl<T> From<T> for Line
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        Self(Uuid::new_v4(), value.into())
    }
}

impl IntoView for Line {
    fn into_view(self) -> View {
        self.1.into_view()
    }
}

impl Line {
    fn id(&self) -> Uuid {
        self.0
    }
}

struct FileSystem(HashMap<String, FileSystemEntry>);

enum FileSystemEntry {
    File(File),
    Directory(Directory),
}

struct File {}

struct Directory {}
