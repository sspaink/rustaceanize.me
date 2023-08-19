use crate::assets::Assets;
use egui::RichText;

struct Body {
    color: String,
    hat: Option<String>,
}

pub struct App {
    assets: Assets,
    ferris: Body,
}

impl Default for App {
    fn default() -> Self {
        Self {
            assets: Assets::new(),
            ferris: Body {
                color: "orange".to_string(),
                hat: None,
            },
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { assets, ferris } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.collapsing(RichText::new("Body").size(20.0), |ui| {
                    egui::Grid::new("bodies").show(ui, |ui| {
                        for (name, image) in assets.colors.iter() {
                            if ui
                                .add(egui::ImageButton::new(
                                    image.thumb.texture_id(ctx),
                                    image.thumb.size_vec2(),
                                ))
                                .clicked()
                            {
                                ferris.color = name.to_string();
                            };
                        }

                        // crab.show(ui);
                        // crab.show(ui);
                        // ui.end_row();
                        // crab.show(ui);
                        // crab.show(ui);
                    });
                });

                // ui.collapsing(RichText::new("Eyes").size(20.0), |ui|{
                //     crab.show(ui);
                // });

                ui.collapsing(RichText::new("Hats").size(20.0), |ui| {
                    egui::Grid::new("hats").show(ui, |ui| {
                        if ui
                            .add(egui::ImageButton::new(
                                assets.remove_thumb.texture_id(ctx),
                                assets.remove_thumb.size_vec2(),
                            ))
                            .clicked()
                        {
                            ferris.hat = None
                        };

                        for (name, image) in assets.hats.iter() {
                            if ui
                                .add(egui::ImageButton::new(
                                    image.thumb.texture_id(ctx),
                                    image.thumb.size_vec2(),
                                ))
                                .clicked()
                            {
                                ferris.hat = Some(name.to_string());
                            };
                        }

                        // crab.show(ui);
                        // crab.show(ui);
                        // ui.end_row();
                        // crab.show(ui);
                        // crab.show(ui);
                    });
                });

                if ui.button("save").clicked() {
                    ui.ctx().output_mut(|o| {
                        o.open_url = Some(egui::output::OpenUrl {
                            url: "https://github.com".to_string(),
                            new_tab: true,
                        });
                    });
                }
            });

        egui::Area::new("body").movable(false).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if let Some(image) = assets.colors.get(&ferris.color) {
                    // TODO how to use show_scaled when window resizes???
                    image.main.show_max_size(ui, ui.available_size());
                }
            });
        });

        egui::Area::new("body").movable(false).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if let Some(hat) = &ferris.hat {
                    if let Some(image) = assets.hats.get(hat) {
                        image.main.show_max_size(ui, ui.available_size());
                    }
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
