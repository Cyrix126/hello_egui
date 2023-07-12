use std::hash::Hash;
use eframe::egui::{Context, Id};
use eframe::{egui, App, Frame, NativeOptions};

use egui_dnd::utils::shift_vec;
use egui_dnd::{DragDropItem, DragDropUi};

struct DnDApp {
    // DragDropUi stores state about the currently dragged item
    dnd: DragDropUi,
    items: Vec<ItemType>,
}

impl Default for DnDApp {
    fn default() -> Self {
        DnDApp {
            dnd: DragDropUi::default(),
            items: ["alfred", "bernhard", "christian"]
                .iter()
                .map(|name| ItemType {
                    name: name.to_string(),
                })
                .collect(),
        }
    }
}

struct ItemType {
    name: String,
}

// We need this to uniquely identify items. You can also implement the Hash trait.
impl DragDropItem for ItemType {
    fn id(&self) -> Id {
        Id::new(&self.name)
    }
}

impl App for DnDApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response =
                // make sure this is called in a vertical layout.
                // Horizontal sorting is not supported yet.
                self.dnd.ui::<ItemType, _>(ui, self.items.iter_mut(), |item, ui, handle, dragging| {
                    ui.horizontal(|ui| {
                        // Anything in the handle can be used to drag the item
                        handle.ui(ui, |ui| {
                            if dragging {
                                ui.label("wheeeeee");
                            } else {
                                ui.label("drag");
                            }
                        });

                        ui.label(&item.name);
                    });
                });

            if let Some(response) = &response.update {
                ui.label(format!("Dragging: {:?}", response));
            }

            // After the drag is complete, we get a response containing the old index of the
            // dragged item, as well as the index it was moved to. You can use the
            // shift_vec function as a helper if you store your items in a Vec.
            if let Some(response) = response.final_update() {
                shift_vec(response.from, response.to, &mut self.items);
            }
        });

        egui::Window::new("Devzg")
            .show(ctx, |ui| {
                ctx.style_ui(ui);
            });
    }
}

pub fn main() {
    eframe::run_native(
        "DnD Example",
        NativeOptions::default(),
        Box::new(|_a| Box::<DnDApp>::default()),
    )
    .unwrap();
}
