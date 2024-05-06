use leptos::*;
use leptos::{ev::SubmitEvent, html::Input};
use shlex::Shlex;
use std::collections::HashMap;
use uuid::Uuid;

#[component]
pub fn Term() -> impl IntoView {
    let input_ref: NodeRef<Input> = create_node_ref();
    let output = create_rw_signal(vec![Line::from("Hello world")]);
    let input = create_rw_signal(String::from(""));
    let current_dir = create_rw_signal(String::from("root"));

    let filesystem: StoredValue<FileSystem> = store_value(
        vec![(
            "root/test_file".into(),
            FileSystemEntry::File {
                name: "test_file".into(),
                content: "this\ntest\nis\na\nmultiline\ntest file".into(),
            },
        )]
        .into(),
    );

    let command_eval = move |ev: SubmitEvent| {
        ev.prevent_default();
        let elem = input_ref.get().unwrap();
        let value = elem.value();
        let mut tokens = Shlex::new(value.as_str());

        let (command, args) = (tokens.next(), tokens.collect::<Vec<_>>());

        output.update(|history| {
            let filesystem = filesystem.get_value();
            if let Some(command) = command {
                history.push(format!("{}> {command} {}", current_dir.get(), args.join(" ")).into());
                match command.as_str() {
                    "cat" => history.push('b: {
                        let Some(target) = args.first() else {
                            break 'b "no argument".into();
                        };
                        if let Some(FileSystemEntry::File { content, .. }) =
                            filesystem.get(&format!("{}/{target}", current_dir.get()))
                        {
                            content.into()
                        } else {
                            format!("file not found: {target}").into()
                        }
                    }),
                    "echo" => history.push(args.join(" ").into()),
                    command => history.push(format!("command not found: {command}").into()),
                };
            }
        });

        input.set("".into())
    };

    view! {
        <div id="term">
            <div>
                <form on:submit=command_eval>
                    <label for="input">{"root>"}</label>
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

#[derive(Clone, Debug)]
struct FileSystem(HashMap<String, FileSystemEntry>);
impl From<Vec<(String, FileSystemEntry)>> for FileSystem {
    fn from(value: Vec<(String, FileSystemEntry)>) -> Self {
        let mut fs = HashMap::new();
        value.into_iter().for_each(|(k, v)| {
            fs.insert(k, v);
        });
        Self(fs)
    }
}

impl FileSystem {
    pub fn get(&self, k: &String) -> Option<&FileSystemEntry> {
        self.0.get(k)
    }

    pub fn get_mut(&mut self, k: &String) -> Option<&mut FileSystemEntry> {
        self.0.get_mut(k)
    }
}

#[derive(Clone, Debug)]
enum FileSystemEntry {
    File { name: String, content: String },
    Directory(Vec<String>),
    Link(String),
}
