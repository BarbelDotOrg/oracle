use cosmic::{Application, Core, Element};
use cosmic::app::Task;
use crate::state::{State, VarKind};
use cosmic::iced::widget as iced_widget; // widget::Id lives here

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    OpenCreateDialog,
    OpenEditDialog(String),
    DialogCancel,
    DialogSave,
    DialogKeyChanged(String),
    DialogKindChanged(VarKind),
    DialogSimpleValueChanged(String),
    DialogPathlikePartChanged(usize, String),
    DialogPathlikePartRemoved(usize),
    DialogNewPartInputChanged(String),
    DialogPathlikePartAdded,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialogMode {
    Create,
    Edit { original_key: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialogState {
    pub mode: DialogMode,
    pub key: String,
    pub kind: VarKind,
    pub simple_value: String,
    pub pathlike_parts: Vec<String>,
    pub new_part_input: String,
}

pub struct OracleApp {
    core: Core,
    pub(crate) state: State,
    dialog: Option<DialogState>,
}

impl Application for OracleApp {
    const APP_ID: &'static str = "org.barbel.Oracle";
    type Message = Message;
    type Flags = ();
    type Executor = cosmic::executor::Default;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (Self {
            core,
            state: State::load().unwrap_or_default(),
            dialog: None,
        }, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::OpenCreateDialog => {
                self.dialog = Some(DialogState {
                    mode: DialogMode::Create,
                    key: String::new(),
                    kind: VarKind::Simple,
                    simple_value: String::new(),
                    pathlike_parts: vec![],
                    new_part_input: String::new(),
                });
            }
            Message::OpenEditDialog(key) => {
                let (kind, simple_value, pathlike_parts) = self.state.describe(&key);
                self.dialog = Some(DialogState {
                    mode: DialogMode::Edit { original_key: key.clone() },
                    key,
                    kind,
                    simple_value,
                    pathlike_parts,
                    new_part_input: String::new(),
                });
            }
            Message::DialogCancel => {
                self.dialog = None;
            }
            Message::DialogSave => {
                if let Some(d) = self.dialog.take() {
                    if !d.key.trim().is_empty() {
                        self.state.save_var(&d.key, d.kind, &d.simple_value, &d.pathlike_parts);
                        let _ = self.state.save();
                    }
                }
            }
            Message::DialogKeyChanged(v) => {
                if let Some(d) = &mut self.dialog { d.key = v; }
            }
            Message::DialogKindChanged(kind) => {
                if let Some(d) = &mut self.dialog { d.kind = kind; }
            }
            Message::DialogSimpleValueChanged(v) => {
                if let Some(d) = &mut self.dialog { d.simple_value = v; }
            }
            Message::DialogPathlikePartChanged(i, v) => {
                if let Some(d) = &mut self.dialog {
                    if let Some(part) = d.pathlike_parts.get_mut(i) { *part = v; }
                }
            }
            Message::DialogPathlikePartRemoved(i) => {
                if let Some(d) = &mut self.dialog {
                    if i < d.pathlike_parts.len() { d.pathlike_parts.remove(i); }
                }
            }
            Message::DialogNewPartInputChanged(v) => {
                if let Some(d) = &mut self.dialog { d.new_part_input = v; }
            }
            Message::DialogPathlikePartAdded => {
                if let Some(d) = &mut self.dialog {
                    if !d.new_part_input.trim().is_empty() {
                        d.pathlike_parts.push(std::mem::take(&mut d.new_part_input));
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.trve_view()
    }

    fn dialog(&self) -> Option<Element<'_, Self::Message>> {
        self.dialog.as_ref().map(|d| self.dialog_view(d))
    }
}