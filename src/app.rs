use crate::assets::{Assets, Image};
use eframe::glow::COLOR;
use egui::{Color32, RichText};
use egui_extras::RetainedImage;
use std::collections::HashMap;

const MAX_ROW: i32 = 2;

struct Body {
    color: Option<String>,
    eyes: Option<String>,
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
                color: Some("orange".to_string()),
                eyes: Some("happy".to_string()),
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

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .min_width(200.0)
            .show(ctx, |ui| {
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

                if ui.button(RichText::new("save").size(20.0)).clicked() {
                    ui.ctx().output_mut(|o| {
                        o.open_url = Some(egui::output::OpenUrl {
                            url: "https://github.com".to_string(),
                            new_tab: true,
                        });
                    });
                }
            });

        display_bodyparts(ctx, &mut ferris.color, &assets.colors);
        display_bodyparts(ctx, &mut ferris.eyes, &assets.eyes);
        display_bodyparts(ctx, &mut ferris.hat, &assets.hats);

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
    ui.collapsing(RichText::new(id).size(20.0), |ui| {
        egui::Grid::new(id).show(ui, |ui| {
            let mut current = 0;

            for (name, image) in assets.iter() {
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
