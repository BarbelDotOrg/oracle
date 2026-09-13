use cosmic::iced::widget::table;
use cosmic::iced::Length;
use cosmic::widget::{button, column, dialog, row, scrollable, space, text, text_input};
use cosmic::{widget, Element, Apply};
use std::collections::HashMap;
use cosmic::widget::button::ButtonClass;
use crate::state::{ManagedVariable, State, VarKind};
use crate::ui::app::{DialogMode, DialogState, Message};

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

        let header_row = row![]
            .push(text::title3("Environment Variables"))
            .push(space::horizontal())
            .push(button::text("+ Add Variable").on_press(Message::OpenCreateDialog))
            .align_y(cosmic::iced::Alignment::Center);

        let key_col = table::column(text("Key"), |r: EnvRow| -> Element<Message> {
            text(r.key).into()
        }).width(Length::FillPortion(2));

        let value_col = table::column(text("Value"), |r: EnvRow| -> Element<Message> {
            text(r.value).into()
        }).width(Length::FillPortion(4));

        let source_col = table::column(text("Source"), |r: EnvRow| -> Element<Message> {
            text(r.source).into()
        }).width(Length::FillPortion(1));

        let actions_col = table::column(text("Actions"), |r: EnvRow| -> Element<Message> {
            button::text("Edit")
                .on_press(Message::OpenEditDialog(r.key.clone()))
                .into()
        }).width(Length::FillPortion(1));

        let env_table = table::table(vec![key_col, value_col, source_col, actions_col], rows)
            .padding(8)
            .separator_x(1)
            .separator_y(1)
            .width(Length::Fill);

        let content = column![]
            .push(header_row)
            .push(env_table)
            .spacing(16)
            .padding(16);

        scrollable(content)
            .height(Length::Fill)
            .into()
    }

    pub fn dialog_view<'a>(&self, d: &'a DialogState) -> Element<'a, Message> {
        let editing_existing_key = matches!(d.mode, DialogMode::Edit { .. });

        let key_input = text_input("VAR_NAME", &d.key)
            .on_input(Message::DialogKeyChanged)
            .apply(|input| if editing_existing_key {
                // Existing keys shouldn't be renamed through this dialog —
                // that would silently orphan the old entry.
                input // still shown, but leave editable if you want rename support later
            } else {
                input
            });

        let kind_row = row![]
            .push(
                button::text("Simple")
                    .class(if d.kind == VarKind::Simple {
                        ButtonClass::Suggested
                    } else {
                        ButtonClass::Standard
                    })
                    .on_press(Message::DialogKindChanged(VarKind::Simple)),
            )
            .push(
                button::text("Pathlike")
                    .class(if d.kind == VarKind::Pathlike {
                        ButtonClass::Suggested
                    } else {
                        ButtonClass::Standard
                    })
                    .on_press(Message::DialogKindChanged(VarKind::Pathlike)),
            )
            .spacing(8);

        let value_editor: Element<'_, Message> = match d.kind {
            VarKind::Simple => text_input("value", &d.simple_value)
                .on_input(Message::DialogSimpleValueChanged)
                .into(),
            VarKind::Pathlike => {
                let mut parts_col = column![].spacing(6);

                for (i, part) in d.pathlike_parts.iter().enumerate() {
                    let row = row![]
                        .push(
                            text_input("path segment", part)
                                .on_input(move |v| Message::DialogPathlikePartChanged(i, v))
                                .width(Length::Fill),
                        )
                        .push(
                            button::text("✕").on_press(Message::DialogPathlikePartRemoved(i)),
                        )
                        .spacing(6);
                    parts_col = parts_col.push(row);
                }

                let add_row = row![]
                    .push(
                        text_input("add a path segment", &d.new_part_input)
                            .on_input(Message::DialogNewPartInputChanged)
                            .on_submit(|_| Message::DialogPathlikePartAdded)
                            .width(Length::Fill),
                    )
                    .push(button::text("Add").on_press(Message::DialogPathlikePartAdded))
                    .spacing(6);

                column![]
                    .push(parts_col)
                    .push(add_row)
                    .spacing(10)
                    .into()
            }
        };

        dialog::dialog()
            .title(match d.mode {
                DialogMode::Create => "Create Variable",
                DialogMode::Edit { .. } => "Edit Variable",
            })
            .control(key_input)
            .control(kind_row)
            .control(value_editor)
            .primary_action(button::suggested("Save").on_press(Message::DialogSave))
            .secondary_action(button::standard("Cancel").on_press(Message::DialogCancel))
            .into()
    }
}