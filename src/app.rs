use egui::RichText;
use egui_extras::RetainedImage;
use std::collections::HashMap;

struct Image {
    main: RetainedImage,
    thumb: RetainedImage,
}

struct Body {
    color: String,
    hat: Option<String>,
}

pub struct App {
    removeThumb: RetainedImage,
    bodies: HashMap<String, Image>,
    hats: HashMap<String, Image>,
    ferris: Body,
}

impl Default for App {
    fn default() -> Self {
        Self {
            removeThumb: RetainedImage::from_image_bytes(
                "remove",
                include_bytes!("../assets/remove_thumb.png"),
            )
            .unwrap(),
            bodies: (|| {
                // TODO: Move to a build script or read from file at startup
                let mut images: HashMap<String, Image> = HashMap::new();

                let image = RetainedImage::from_image_bytes(
                    "purple",
                    include_bytes!("../assets/colors/purple.png"),
                )
                .unwrap();

                let thumb = RetainedImage::from_image_bytes(
                    "orange",
                    include_bytes!("../assets/colors/purple_thumb.png"),
                )
                .unwrap();
                images.insert(
                    "purple".to_string(),
                    Image {
                        main: image,
                        thumb: thumb,
                    },
                );

                let image = RetainedImage::from_image_bytes(
                    "orange",
                    include_bytes!("../assets/colors/orange.png"),
                )
                .unwrap();

                let thumb = RetainedImage::from_image_bytes(
                    "orange",
                    include_bytes!("../assets/colors/orange_thumb.png"),
                )
                .unwrap();
                images.insert(
                    "orange".to_string(),
                    Image {
                        main: image,
                        thumb: thumb,
                    },
                );

                images
            })(),
            hats: (|| {
                // TODO: Move to a build script or read from file at startup
                let mut images: HashMap<String, Image> = HashMap::new();

                let image = RetainedImage::from_image_bytes(
                    "top",
                    include_bytes!("../assets/hats/top.png"),
                )
                .unwrap();
                let thumb = RetainedImage::from_image_bytes(
                    "orange",
                    include_bytes!("../assets/hats/top_thumb.png"),
                )
                .unwrap();
                images.insert(
                    "top".to_string(),
                    Image {
                        main: image,
                        thumb: thumb,
                    },
                );

                images
            })(),
            ferris: Body {
                color: "orange".to_string(),
                hat: Some("top".to_string()),
            },
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App {
    // /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            removeThumb,
            bodies,
            hats,
            ferris,
        } = self;

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
                        for (name, image) in bodies.iter() {
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
                                removeThumb.texture_id(ctx),
                                removeThumb.size_vec2(),
                            ))
                            .clicked()
                        {
                            ferris.hat = None
                        };

                        for (name, image) in hats.iter() {
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

        egui::Area::new("my_area").movable(false).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if let Some(image) = bodies.get(&ferris.color) {
                    // TODO how to use show_scaled when window resizes???
                    image.main.show(ui);
                }
            });
        });

        egui::Area::new("my_area").movable(false).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if let Some(hat) = &ferris.hat {
                    if let Some(image) = hats.get(hat) {
                        image.main.show(ui);
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
