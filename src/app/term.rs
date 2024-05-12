use leptos::*;
use leptos::{
    ev::{KeyboardEvent, SubmitEvent},
    html::Input,
};
use shlex::Shlex;
use std::collections::{hash_map::Iter, HashMap};
use uuid::Uuid;

#[component]
pub fn Term() -> impl IntoView {
    let input_ref: NodeRef<Input> = create_node_ref();
    let output = create_rw_signal(vec![Line::from("Hello world")]);
    let input = create_rw_signal(String::from(""));
    let current_dir = create_rw_signal(String::from("root"));
    let cmp_results: RwSignal<Option<String>> = create_rw_signal(None);

    let shlex = move || {
        let elem = input_ref.get().unwrap();
        let value = elem.value();
        let mut tokens = Shlex::new(value.as_str());
        (tokens.next(), tokens.collect::<Vec<String>>())
    };

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

    type Cmd = Box<dyn Fn(Vec<&str>) -> Line>;

    let ls: Cmd = Box::new(move |_| {
        let filesystem = filesystem.get_value();
        filesystem
            .iter()
            .filter_map(|(path, entry)| {
                if path.as_str() == format!("{}/{}", current_dir.get(), entry.name()) {
                    Some(entry.name())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    });

    let cat: Cmd = Box::new(move |args| {
        let filesystem = filesystem.get_value();
        if let Some(target) = args.first() {
            match filesystem.get(&format!("{}/{target}", current_dir.get())) {
                Some(FileSystemEntry::File { content, .. }) => content.into(),
                _ => format!("file not found: {target}").into(),
            }
        } else {
            "no argument".into()
        }
    });

    let echo: Cmd = Box::new(move |args| Line::from(args.join(" ")));

    let cmds: HashMap<&str, Cmd> = HashMap::from([("ls", ls), ("cat", cat), ("echo", echo)]);

    let command_eval = move |ev: SubmitEvent| {
        ev.prevent_default();
        let (command, args) = shlex();

        output.update(|history| {
            if let Some(command) = command {
                history.push(format!("{}> {command} {}", current_dir.get(), args.join(" ")).into());
                history.push(cmds.get(command.as_str()).map_or_else(
                    || format!("command not found: {command}").into(),
                    |f| f(args.iter().map(String::as_str).collect()),
                ));
            }
        });

        input.set("".into())
    };

    let key_action = move |ev: KeyboardEvent| {
        if ev.key_code() == 9 {
            ev.prevent_default();
            ev.stop_propagation();
        }
    };

    view! {
        <div id="term">
            {cmp_results}
            <div>
                <form on:submit=command_eval on:keydown=key_action>
                    <label for="input">{move || format!("{}>", current_dir.get())}</label>
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
        Self(HashMap::from_iter(value))
    }
}

impl FileSystem {
    pub fn get(&self, k: &str) -> Option<&FileSystemEntry> {
        self.0.get(k)
    }

    pub fn get_mut(&mut self, k: &str) -> Option<&mut FileSystemEntry> {
        self.0.get_mut(k)
    }

    pub fn iter(&self) -> Iter<String, FileSystemEntry> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
enum FileSystemEntry {
    File { name: String, content: String },
    Directory { name: String, content: Vec<String> },
    Link { name: String, content: String },
}

impl FileSystemEntry {
    fn name(&self) -> &str {
        match self {
            Self::File { name, .. } | Self::Link { name, .. } => name,
            Self::Directory { name, .. } => name,
        }
    }
}
