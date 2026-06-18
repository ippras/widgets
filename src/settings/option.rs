use egui::{ComboBox, Response, Ui, Widget};
use egui_variant::{EnumStyle, Style};
use std::mem::swap;

/// Option variant
#[derive(TypedBuilder)]
pub struct OptionVariant<'a, T, F> {
    pub value: &'a mut Option<T>,
    pub default_some: F,
    #[builder(default)]
    pub style: Option<EnumStyle>,
}

impl<'a, T, F> OptionVariant<'a, T, F>
where
    T: Widget,
    F: FnMut() -> T,
{
    fn ui(mut self, ui: &mut Ui) -> impl FnOnce(&mut Ui, &Style) -> Response {
        let Self {
            value,
            mut default_some,
            style: option_style,
        } = self;
        let mut default = Some(default_some());
        move |ui, style| {
            let mut changed = false;
            let mut response = ui
                .horizontal(|ui| {
                    let mut checked = value.is_some();
                    match option_style.unwrap_or(style.variants) {
                        EnumStyle::ComboBox => {
                            ComboBox::from_id_salt(ui.make_persistent_id("ComboBox"))
                                .width(ui.style().spacing.combo_width / 2.0)
                                .selected_text(match value {
                                    None => "Disable",
                                    Some(_) => "Enable",
                                })
                                .show_ui(ui, |ui| {
                                    if ui.selectable_label(!checked, "Disable").clicked() {
                                        checked = false;
                                    }
                                    if ui.selectable_label(checked, "Enable").clicked() {
                                        checked = true;
                                    }
                                });
                        }
                        EnumStyle::Horizontal => {
                            ui.horizontal(|ui| {
                                if ui.selectable_label(!checked, "Disable").clicked() {
                                    checked = false;
                                }
                                if ui.selectable_label(checked, "Enable").clicked() {
                                    checked = true;
                                }
                            });
                        }
                    }

                    match (checked, value.is_some()) {
                        (true, false) => {
                            swap(value, &mut default);
                            // *value = Some(default_some);
                            changed = true;
                        }
                        (false, true) => {
                            default = value.take();
                            changed = true;
                        }
                        _ => {}
                    }

                    // let id = ui.next_auto_id();
                    if let Some(value) = &mut value {
                        if value.ui(ui).changed() {
                            // State::new(value.clone()).store(ui, id);
                            changed = true;
                        }
                    } else {
                        ui.disable();
                        // let mut state = State::load(ui, id).unwrap_or_else(|| State::new(default_some()));
                        // probe(&mut state.default_some, ui, style);
                        if let Some(default) = &mut default {
                            // default.ui(ui);
                        }
                    }
                })
                .response;

            if changed {
                response.mark_changed();
            }

            response
        }
    }
}
