use cosmic::Element;
use crate::ui::app::{Message, OracleApp};

impl OracleApp {
   pub fn trve_view(&self) -> Element<'_, Message> {
        cosmic::widget::Text::new("Hello, world!").into()
    }
}