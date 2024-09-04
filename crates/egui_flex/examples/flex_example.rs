use eframe::NativeOptions;
use egui::{Button, CentralPanel, Checkbox, Label, Slider, TextEdit, Widget};
use egui_flex::flex_button::FlexButton;
use egui_flex::{Flex, FlexItem};

fn main() -> eframe::Result {
    let mut text = "Hello, world!".to_owned();
    eframe::run_simple_native(
        "flex example",
        NativeOptions::default(),
        move |ctx, _frame| {
            CentralPanel::default().show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    let items = vec![
                        "I",
                        "can have veeeeeeeeeeeeery long",
                        "and",
                        "very",
                        "short",
                        "and",
                        "multi\nline",
                        "or\neven\nmore\nlines",
                        "and",
                        "even",
                        "some middle length",
                        "items",
                    ];

                    Flex::new()
                        .align_items(egui_flex::FlexAlign::Stretch)
                        .align_item_content(egui::Align2::CENTER_CENTER)
                        .show(ui, |flex| {
                            flex.add_container(FlexItem::default().grow(1.0), |ui, content| {
                                ui.group(|ui| {
                                    content.content(ui, |ui| {
                                        ui.label("Hello");
                                    })
                                })
                                .inner
                            });

                            for item in items {
                                flex.add_container(FlexItem::default().grow(1.0), |ui, content| {
                                    ui.group(|ui| {
                                        content.content(ui, |ui| {
                                            Label::new(item).wrap().ui(ui);
                                        })
                                    })
                                    .inner
                                });
                            }

                            flex.add_container(
                                FlexItem::default().grow(1.0).basis(400.0),
                                |ui, content| {
                                    ui.group(|ui| {
                                        content.content(ui, |ui| {
                                            TextEdit::singleline(&mut text)
                                                .desired_width(ui.available_width())
                                                .ui(ui);
                                        })
                                    })
                                    .inner
                                },
                            );

                            flex.add_container(
                                FlexItem::default().grow(1.0).basis(100.0),
                                |ui, content| {
                                    ui.group(|ui| {
                                        content.content(ui, |ui| {
                                            ui.add(Label::new("I have flex basis 100").wrap());
                                        })
                                    })
                                    .inner
                                },
                            );
                            flex.add_simple(FlexItem::new().grow(1.0).basis(100.0), |ui| {
                                ui.style_mut().spacing.slider_width = ui.available_width() - 50.0;
                                Slider::new(&mut 0.0, 0.0..=1000.0).ui(ui);
                            });

                            flex.add(FlexItem::new().grow(1.0), FlexButton::new("Button"));
                            flex.add(
                                FlexItem::new().grow(1.0),
                                FlexButton::new("Button wefoijfgiweopjg"),
                            );
                            flex.add(FlexItem::new().grow(1.0), FlexButton::new("Button"));
                            flex.add_widget(FlexItem::new(), Button::new("Simple Button"));

                            flex.add_widget(FlexItem::new(), Checkbox::new(&mut false, "Checkbox"));

                            // flex.add_container(
                            //     FlexItem::default().grow(1.0).basis(100.0),
                            //     |ui, content| {
                            //         ui.group(|ui| {
                            //             content.content(ui, |ui| {
                            //                 ui.vertical(|ui| {
                            //                     Flex::new().show(ui, |flex| {
                            //                         flex.add(
                            //                             FlexItem::new(),
                            //                             FlexButton::new("Button"),
                            //                         );
                            //
                            //                         flex.add(
                            //                             FlexItem::new(),
                            //                             FlexButton::new("Longer Button"),
                            //                         );
                            //
                            //                         flex.add(
                            //                             FlexItem::new(),
                            //                             FlexButton::new(
                            //                                 "Button\nwith\nmultiple\nlines",
                            //                             ),
                            //                         );
                            //                     });
                            //                 });
                            //             })
                            //         })
                            //         .inner
                            //     },
                            // );
                        });
                });

                // ui.horizontal_top(|ui| {
                //     Flex::new().show(ui, |flex| {
                //         for i in 0..1000 {
                //             flex.add_simple(FlexItem::default().grow(1.0), |ui| {
                //                 ui.label(i.to_string());
                //             });
                //         }
                //         flex.add_simple(FlexItem::default().grow(1000000.0), |ui| {});
                //     });
                // });

                ui.button("Button");
            });
        },
    )
}
