use cosmic::iced::widget::table;
use cosmic::iced::Length;
use cosmic::widget::{column, scrollable, text};
use cosmic::Element;
use std::collections::HashMap;
use crate::state::{ManagedVariable, State};
use crate::ui::app::Message;

#[derive(Clone)]
struct EnvRow {
    key: String,
    value: String,
    source: &'static str,
}

impl State {
    fn all_rows(&self) -> Vec<EnvRow> {
        let mut merged: HashMap<String, (String, &'static str)> = HashMap::new();

        for (k, v) in &self.env_vars {
            merged.insert(k.clone(), (v.clone(), "System"));
        }
        for (k, var) in &self.vars {
            let value = match var {
                ManagedVariable::Simple(s) => s.clone(),
                ManagedVariable::Pathlike(vec) => vec.join(":"),
            };
            merged.insert(k.clone(), (value, "Managed"));
        }
        for (k, v) in &self.overrides {
            merged.insert(k.clone(), (v.clone(), "Override"));
        }
        for (k, adds) in &self.pathlike_adds {
            if let Some((value, _)) = merged.get_mut(k) {
                *value = format!("{value}:{}", adds.join(":"));
            } else {
                merged.insert(k.clone(), (adds.join(":"), "Managed"));
            }
        }

        let mut rows: Vec<EnvRow> = merged
            .into_iter()
            .map(|(key, (value, source))| EnvRow { key, value, source })
            .collect();
        rows.sort_by(|a, b| a.key.cmp(&b.key));
        rows
    }
}

impl crate::ui::app::OracleApp {
    pub fn trve_view(&self) -> Element<'_, Message> {
        let rows = self.state.all_rows();

        let key_col = table::column(text("Key"), |r: EnvRow| -> Element<Message> {
            text(r.key).into()
        })
            .width(Length::FillPortion(2));

        let value_col = table::column(text("Value"), |r: EnvRow| -> Element<Message> {
            text(r.value).into()
        })
            .width(Length::FillPortion(4));

        let source_col = table::column(text("Source"), |r: EnvRow| -> Element<Message> {
            text(r.source).into()
        })
            .width(Length::FillPortion(1));

        let env_table = table::table(vec![key_col, value_col, source_col], rows)
            .padding(8)
            .separator_x(1)
            .separator_y(1)
            .width(Length::Fill);

        let content = column![]
            .push(env_table)
            .spacing(16)
            .padding(16);

        scrollable(content)
            .height(Length::Fill)
            .into()
    }
}