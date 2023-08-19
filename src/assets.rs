use egui_extras::RetainedImage;
use std::collections::HashMap;

pub struct Image {
    pub main: RetainedImage,
    pub thumb: RetainedImage,
}

pub struct Assets {
    pub colors: HashMap<String, Image>,
    pub hats: HashMap<String, Image>,
    pub remove_thumb: RetainedImage,
}

macro_rules! load_image {
    ($path:expr, $file:expr) => {
        (
            $file.to_string(),
            Image {
                main: RetainedImage::from_image_bytes(
                    $file,
                    include_bytes!(concat!("../assets/", $path, "/", $file, ".png")),
                )
                .unwrap(),
                thumb: RetainedImage::from_image_bytes(
                    concat!($file, "_thumb"),
                    include_bytes!(concat!("../assets/", $path, "/", $file, "_thumb.png")),
                )
                .unwrap(),
            },
        )
    };
}

impl Assets {
    pub fn new() -> Assets {
        let mut colors: HashMap<String, Image> = HashMap::new();

        let images = [
            load_image!("colors", "purple"),
            load_image!("colors", "orange"),
        ];

        for i in images {
            colors.insert(i.0, i.1);
        }

        let mut hats: HashMap<String, Image> = HashMap::new();

        let images = [load_image!("hats", "top")];

        for i in images {
            hats.insert(i.0, i.1);
        }

        let remove_thumb =
            RetainedImage::from_image_bytes("remove", include_bytes!("../assets/remove_thumb.png"))
                .unwrap();

        Assets {
            colors,
            hats,
            remove_thumb,
        }
    }
}
