use std::{collections::HashMap, fs};

use image::{io::Reader as ImageReader, DynamicImage};

fn load_images(category: &str) -> HashMap<String, DynamicImage> {
    let mut images = HashMap::new();

    let paths = fs::read_dir(format!("../assets/{category}")).unwrap();
    for entry in paths {
        let path = entry.unwrap().path();
        if !path.to_str().unwrap().ends_with("_thumb.png") {
            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
            images.insert(
                name,
                ImageReader::open(path.as_path()).unwrap().decode().unwrap(),
            );
        }
    }

    images
}

fn merge(
    images1: &HashMap<String, DynamicImage>,
    images2: &HashMap<String, DynamicImage>,
) -> HashMap<String, DynamicImage> {
    let mut merged = HashMap::new();

    for (color_name, color_image) in images1 {
        for (eye_name, eye_image) in images2 {
            let mut base = color_image.clone();
            image::imageops::overlay(&mut base, eye_image, 0, 0);
            merged.insert(format!("{}_{}", color_name, eye_name), base);
        }
    }

    merged
}

fn main() {
    const ROOT_DIR: &str = "../pregen_crabs";
    fs::remove_dir_all(ROOT_DIR).unwrap();
    fs::create_dir(ROOT_DIR).unwrap();

    let colors = load_images("colors");
    let eyes: HashMap<String, DynamicImage> = load_images("eyes");

    let mut merged = HashMap::new();

    merged.extend(merge(&colors, &eyes));

    let hats = load_images("hats");

    merged.extend(merge(&merged, &hats));

    for (name, image) in merged {
        image.save(format!("{ROOT_DIR}/{name}.png")).unwrap();
    }
}
