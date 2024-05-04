use egui::{Response, RichText, Widget};
use std::borrow::{Borrow, Cow};
use std::hash::Hash;

mod form;
#[cfg(feature = "validator_garde")]
pub mod garde;
mod validation_error;
#[cfg(feature = "validator_validator")]
pub mod validator;

use crate::form::FormFieldState;
pub use form::Form;
pub use validation_error::EguiValidationErrors;

pub struct FormField<'a, 'f, Errors: EguiValidationErrors> {
    error: Option<String>,
    label: Option<Cow<'a, str>>,
    form: Option<&'f mut Form<Errors>>,
}

impl<'a, 'f, Errors: EguiValidationErrors> FormField<'a, 'f, Errors> {
    pub fn new<B: Eq + Hash + Ord + ?Sized>(form: &'f mut form::Form<Errors>, field: &B) -> Self
    where
        Errors::Check: Borrow<B>,
    {
        let error = form
            .validation_results
            .iter()
            .find_map(|errors| errors.get_field_error(field));

        FormField {
            error,
            label: None,
            form: Some(form),
        }
    }

    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn ui(self, ui: &mut egui::Ui, content: impl Widget) -> Response {
        let error = self.error;

        ui.vertical(|ui| {
            let id = ui.auto_id_with("form_field");
            let blurred = ui.memory_mut(|mem| *mem.data.get_temp_mut_or(id, false));

            let error_color = ui.style().visuals.error_fg_color;

            let show_error = error.is_some() && blurred;

            if show_error {
                let widgets = &mut ui.style_mut().visuals.widgets;
                widgets.inactive.bg_stroke.color = error_color;
                widgets.inactive.bg_stroke.width = 1.0;
                widgets.active.bg_stroke.color = error_color;
                widgets.active.bg_stroke.width = 1.0;
                widgets.hovered.bg_stroke.color = error_color;
                widgets.hovered.bg_stroke.width = 1.0;
                widgets.open.bg_stroke.color = error_color;
                widgets.open.bg_stroke.width = 1.0;
            }

            if let Some(label) = self.label {
                let mut rich_text = RichText::new(label.as_ref());
                if show_error {
                    rich_text = rich_text.color(error_color);
                }
                ui.label(rich_text);
            }

            let response = content.ui(ui);

            if response.lost_focus() {
                ui.memory_mut(|mem| {
                    mem.data.insert_temp(id, true);
                });
            }

            if let Some(form) = self.form {
                if let Some(error) = &error {
                    form.controls.push(FormFieldState {
                        state_id: id,
                        widget_id: response.id,
                        errors: vec![error.clone()],
                    });
                } else {
                    form.controls.push(FormFieldState {
                        state_id: id,
                        widget_id: response.id,
                        errors: vec![],
                    });
                };
            }

            ui.add_visible(
                show_error,
                egui::Label::new(
                    RichText::new(error.as_deref().unwrap_or(""))
                        .color(error_color)
                        .small(),
                ),
            );

            response
        })
        .inner
    }
}
