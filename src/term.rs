use std::ops::{Deref, DerefMut};
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use phf::phf_map;
use uuid::Uuid;

#[derive(Clone)]
struct Row(Uuid, String);

#[derive(Clone)]
struct Rows(Vec<Row>);

impl<T> From<T> for Row where String: From<T>
{
    fn from(value: T) -> Self {
        Self(Uuid::new_v4(), value.into())
    }
}

impl<T> From<Vec<T>> for Rows where String: From<T>, {
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().map(Row::from).collect())
    }
}

impl Deref for Rows {
    type Target = Vec<Row>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rows {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
enum Command {
    Echo(Rows),
    Help(Rows),
    NotFound(Rows),
    Cat(Option<String>),
    Ls,
    None,
}

impl Command {
    fn evaluate(self) -> Evaluated {
        match self {
            Self::Echo(data) |
            Self::Help(data) |
            Self::NotFound(data) => {
                Evaluated::TermOutput(data)
            }
            Self::Cat(data) => Evaluated::ReadFile(data),
            Self::Ls => Evaluated::ListDir,
            Self::None => Evaluated::None
        }
    }
}

enum Evaluated {
    TermOutput(Rows),
    ChangedDirectory(String),
    Navigation(String),
    ReadFile(Option<String>),
    ListDir,
    None,
}

static HELP_TEXT: phf::Map<&'static str, &'static str> = phf_map! {
    "echo" => "prints input string to the terminal",
    "ls" => "list current directory contents",
    "cat" => "read the contents of a file",
    "help" => "get some help"
};

impl From<String> for Command {
    fn from(value: String) -> Self {
        let data: Vec<String> = value.split_whitespace().map(|s| s.into()).collect();
        match data.first() {
            Some(s) if s.as_str() == "echo" => Self::Echo(vec![
                "echo:".to_string(),
                format!("  {}", if data.len() > 1 { &value[5..] } else { &"no input" }),
            ].into()),
            Some(s) if s.as_str() == "help" => Self::Help(if data.len() > 1 {
                vec![
                    format!("help: {}", data[1].as_str()),
                    format!("  {}", *HELP_TEXT.get(data[1].as_str()).unwrap_or(&"command not found")),
                ].into()
            } else {
                let mut ret: Rows = vec!["help:"].into();
                ret.append(&mut HELP_TEXT.entries().map(|(k, v)| format!("  {k}: {v}").into()).collect());
                ret
            }),
            Some(s) if s.as_str() == "ls" => Self::Ls,
            Some(s) if s.as_str() == "cat" => Self::Cat(data.get(1).cloned()),
            Some(_) => Self::NotFound(vec![
                "command not found:".to_string(),
                format!("  {value}"),
            ].into()),
            _ => Self::None
        }
    }
}

#[derive(Clone)]
enum File {
    Text(Vec<String>),
    Link(String),
}

#[derive(Clone)]
enum FileDir {
    File { name: String, content: File },
    Directory { name: String, content: Vec<FileDir> },
}

impl FileDir {
    fn name(&self) -> &String {
        match self {
            Self::File { name, .. } |
            Self::Directory { name, .. } => name
        }
    }

    fn content(&self) -> Vec<String> {
        match self {
            Self::File { content: File::Text(lines), .. } => lines.clone(),
            Self::File { content: File::Link(link), .. } => vec![link.clone()],
            Self::Directory { content, .. } => {
                content.iter().map(|fd| fd.name().clone()).collect()
            }
        }
    }
}

fn filesystem_init() -> FileDir {
    FileDir::Directory {
        name: "home".into(),
        content: vec![
            FileDir::File {
                name: "test_file".into(),
                content: File::Text(vec![
                    "this is",
                    "a multi line",
                    "test file",
                ].into_iter().map(String::from).collect()),
            }
        ],
    }
}

fn input_size() -> u16 {
    match window().inner_width().unwrap().as_f64().unwrap() as u16 {
        0..=850 => 50,
        851..=1100 => 70,
        _ => 80,
    }
}

#[component]
pub fn Term() -> impl IntoView {
    let navigate = leptos_router::use_navigate();
    let ((input_r, input_w), input_ref) =
        (create_signal(String::new()), create_node_ref::<Input>());
    let term_output = create_rw_signal(Rows::from(vec!["hello world"]));
    let term_size = create_rw_signal(0);
    let handle = window_event_listener(ev::resize, move |_| term_size.set(input_size()));
    let filesystem = filesystem_init();
    let current_dir = create_rw_signal(filesystem);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let cmd = Command::from(input_r.get());
        match cmd.evaluate() {
            Evaluated::TermOutput(mut output) => {
                term_output.update(|term| term.append(&mut output))
            }
            Evaluated::ListDir => {
                term_output.update(|term| {
                    term.push("ls:".into());
                    term.append(&mut current_dir.get().content().into_iter()
                        .map(|s| ("  ".to_string() + s.as_str()).into()).collect())
                })
            }
            Evaluated::ReadFile(Some(name)) => {

            }
            Evaluated::ReadFile(None) => {
                term_output.update(|term| term.append(&mut Rows::from(vec!["cat:", "  no input"])))
            }
            _ => {}
        };
        input_w.set("".into());
        input_ref.get().unwrap().set_value("");
    };

    create_effect(move |_| term_size.set(input_size()));
    on_cleanup(move || handle.remove());
    view! {
        <div id="term">
            <ul>
                <For
                    each=move || { term_output.get().0.into_iter().rev() }
                    key=|line| line.0
                    children=move |line| {
                        view! { <code>{line.1}</code> }
                    }
                />

            </ul>
            <form on:submit=on_submit>
                <label>{format!("{}~> ", current_dir.get().name())}</label>
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
