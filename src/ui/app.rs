use cosmic::{Application, Core, Element};
use cosmic::app::Task;
use crate::state::State;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {

}

pub struct OracleApp {
    core: Core,
    state: State,
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
            state: State::load().unwrap_or_default()
        }, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.trve_view()
    }
}