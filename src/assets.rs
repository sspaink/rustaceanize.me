use egui_extras::RetainedImage;
use std::collections::HashMap;

pub struct Image {
    pub main: RetainedImage,
    pub thumb: RetainedImage,
}

pub struct Assets {
    pub total_combinations: u64,
    pub colors: HashMap<String, Image>,
    pub eyes: HashMap<String, Image>,
    pub hats: HashMap<String, Image>,
    pub facial_hair: HashMap<String, Image>,
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
        let mut category_count = Vec::new();

        let images = [
            load_image!("colors", "purple"),
            load_image!("colors", "orange"),
            load_image!("colors", "green"),
            load_image!("colors", "blue"),
        ];
        category_count.push(images.len());

        for i in images {
            colors.insert(i.0, i.1);
        }

        let mut eyes: HashMap<String, Image> = HashMap::new();

        let images = [
            load_image!("eyes", "happy"),
            load_image!("eyes", "girl"),
            load_image!("eyes", "angry"),
            load_image!("eyes", "pirate"),
        ];
        category_count.push(images.len());

        for i in images {
            eyes.insert(i.0, i.1);
        }

        let mut hats: HashMap<String, Image> = HashMap::new();

        let images = [
            load_image!("hats", "top"),
            load_image!("hats", "bow"),
            load_image!("hats", "fez"),
        ];
        category_count.push(images.len() + 1);

        for i in images {
            hats.insert(i.0, i.1);
        }

        let mut facial_hair: HashMap<String, Image> = HashMap::new();

        let images = [
            load_image!("facial_hair", "mustache"),
            load_image!("facial_hair", "bushy"),
        ];
        category_count.push(images.len() + 1);

        for i in images {
            facial_hair.insert(i.0, i.1);
        }

        let remove_thumb =
            RetainedImage::from_image_bytes("remove", include_bytes!("../assets/remove_thumb.png"))
                .unwrap();

        let mut total_combinations: u64 = 1;

        for c in category_count {
            total_combinations *= c as u64;
        }

        Assets {
            total_combinations,
            colors,
            eyes,
            hats,
            facial_hair,
            remove_thumb,
        }
    }
}
