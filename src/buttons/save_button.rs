use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::FLOPPY_DISK;
use typed_builder::TypedBuilder;

/// Save button widget
#[derive(Debug, TypedBuilder)]
pub struct SaveButton<'a> {
    atom: &'a str,
    #[builder(default, setter(strip_option))]
    hover: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for SaveButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = (
            RichText::new(FLOPPY_DISK),
            RichText::new(ui.localize(self.atom)),
        );
        atoms = if let Some(size) = self.size {
            (atoms.0.size(size), atoms.1.size(size))
        } else {
            (atoms.0.heading(), atoms.1.heading())
        };
        let mut response = ui.button(atoms);
        if let Some(hover) = self.hover {
            response = response.on_hover_localized(hover)
        }
        response
    }
}

// /// Save button widget
// #[derive(Debug, TypedBuilder)]
// pub struct SaveButton<T: FnOnce(&mut Ui) -> U, U> {
//     add_contents: T,
//     #[builder(default, setter(strip_option))]
//     size: Option<f32>,
// }

// impl<T: FnOnce(&mut Ui) -> U, U> Widget for SaveButton<T, U> {
//     fn ui(self, ui: &mut Ui) -> Response {
//         let mut atoms = RichText::new(FLOPPY_DISK);
//         atoms = if let Some(size) = self.size {
//             atoms.size(size)
//         } else {
//             atoms.heading()
//         };
//         ui.menu_button(atoms, self.add_contents)
//             .response
//             .on_hover_localized("Save")
//     }
// }
