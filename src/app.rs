use crate::assets::{Assets, Image};
use egui::RichText;
use egui_extras::RetainedImage;
use std::collections::HashMap;
use struct_iterable::Iterable;

const MAX_ROW: i32 = 2;

#[derive(Iterable)]
struct Body {
    color: Option<String>,
    eyes: Option<String>,
    hat: Option<String>,
    facial_hair: Option<String>,
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
                color: Some("orange".to_string()),
                eyes: Some("happy".to_string()),
                hat: None,
                facial_hair: None,
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

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .min_width(200.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    display_thumbnails(
                        "Color",
                        &mut ferris.color,
                        ctx,
                        ui,
                        &assets.colors,
                        false,
                        &assets.remove_thumb,
                    );
                    display_thumbnails(
                        "Eyes",
                        &mut ferris.eyes,
                        ctx,
                        ui,
                        &assets.eyes,
                        false,
                        &assets.remove_thumb,
                    );
                    display_thumbnails(
                        "Hat",
                        &mut ferris.hat,
                        ctx,
                        ui,
                        &assets.hats,
                        true,
                        &assets.remove_thumb,
                    );
                    display_thumbnails(
                        "Facial Hair",
                        &mut ferris.facial_hair,
                        ctx,
                        ui,
                        &assets.facial_hair,
                        true,
                        &assets.remove_thumb,
                    );
                });

                ui.vertical_centered_justified(|ui| {
                    if ui.button(RichText::new("save").size(30.0)).clicked() {

                        let mut name: String = "".to_owned();

                        for (field_name, value) in ferris.iter() {
                            if let Some(string_opt) = value.downcast_ref::<Option<String>>() {
                                if let Some(string) = string_opt.as_deref() {
                                    if field_name == "color" {
                                        name += string;
                                    } else {
                                        name += &format!("_{}", string);
                                    }
                                }
                            }
                        }

                        ui.ctx().output_mut(|o| {
                            o.open_url = Some(egui::output::OpenUrl {
                                url: format!("https://raw.githubusercontent.com/sspaink/rustaceanize.me/master/pregen_crabs/{name}.png"),
                                new_tab: true,
                            });
                        });
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!("There are {} possible combinations!", assets.total_combinations));
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.hyperlink_to("View on Github", "https://github.com/sspaink/rustaceanize.me");
                    });
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Inspired by  ");
                        ui.hyperlink_to("gopherize.me", "https://gopherize.me/");
                    });
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Created by  ");
                        ui.hyperlink_to("Sebastian Spaink", "https://www.linkedin.com/in/sebastianspaink/");
                    });
                });
            });

        display_bodyparts(ctx, &mut ferris.color, &assets.colors);
        display_bodyparts(ctx, &mut ferris.eyes, &assets.eyes);
        display_bodyparts(ctx, &mut ferris.hat, &assets.hats);
        display_bodyparts(ctx, &mut ferris.facial_hair, &assets.facial_hair);

        egui::CentralPanel::default().show(ctx, |_| {});
    }
}

fn display_thumbnails(
    id: &str,
    active: &mut Option<String>,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    assets: &HashMap<String, Image>,
    removable: bool,
    remove_thumb: &RetainedImage,
) {
    ui.collapsing(RichText::new(id).size(20.0).strong(), |ui| {
        egui::Grid::new(id).show(ui, |ui| {
            let mut current = 0;
            if removable {
                current += 1;
            }
            if removable
                && ui
                    .add(egui::ImageButton::new(
                        remove_thumb.texture_id(ctx),
                        remove_thumb.size_vec2(),
                    ))
                    .clicked()
            {
                *active = None
            };

            for (name, image) in assets.iter() {
                if ui
                    .add(egui::ImageButton::new(
                        image.thumb.texture_id(ctx),
                        image.thumb.size_vec2(),
                    ))
                    .clicked()
                {
                    *active = Some(name.clone());
                };

                current += 1;

                if current == MAX_ROW {
                    ui.end_row();
                    current = 0;
                }
            }
        });
    });
}

fn display_bodyparts(
    ctx: &egui::Context,
    current: &mut Option<String>,
    assets: &HashMap<String, Image>,
) {
    egui::Area::new("body").movable(false).show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            if let Some(hat) = &current {
                if let Some(image) = assets.get(hat) {
                    image.main.show_max_size(ui, ui.available_size());
                }
            }
        });
    });
}
