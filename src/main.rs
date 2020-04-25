use image::{RgbImage};
use image::imageops;
use rand::prelude::*;
use palette::{Srgba, Srgb, LinSrgba, LinSrgb, Hsva, Hsv, Mix};

#[allow(dead_code)]
fn parse_hsv(s: String) -> Result<Hsv, Box<dyn std::error::Error>> {
    let mut raw = s.split(',').map(|x| x.parse::<f32>());
    Ok(Hsv::new(raw.next().ok_or("expected value for color")??,
        raw.next().ok_or("expected value for color")??,
        raw.next().ok_or("expected value for color")??))
}

enum ColorMode {
    Solid(LinSrgb),
    Random
}

impl ColorMode {
    fn pick_color(&self) -> LinSrgb {
        match self {
            ColorMode::Solid(c) => *c,
            ColorMode::Random => Hsv::new(random::<f32>().powf(1.5) * 360.0, 0.9, random::<f32>()*0.2 + 0.3).into()
        }
    }
}

enum AlphaMode {
    Normal,
    Inverse
}

impl AlphaMode {
    fn calculate_mix_param(&self, src_alpha: f32) -> f32 {
        match self {
            Self::Normal => src_alpha,
            Self::Inverse => 1.0 - src_alpha
        }
    }
}

// usage: truchet-tiles <base tile image> <width> <height> <outputname> (<color-mode> (<HSV color>)) (<alpha-mode>)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let base_tile = image::open(args.next().ok_or("expected tile image path")?)?.to_rgba();
    let image_width = args.next().ok_or("expected image width")?.parse::<u32>()?;
    let image_height = args.next().ok_or("expected image height")?.parse::<u32>()?;
    let output_path = args.next().ok_or("expected output path")?;
    let color_mode = if let Some(color_mode) = args.next() {
        match color_mode.as_str() {
            "solid" => {
                ColorMode::Solid(parse_hsv(args.next().ok_or("expected color for solid color")?)?.into())
            },
            "random" => ColorMode::Random,
            _ => panic!("unknown color mode {}", color_mode)
        }
    } else {
        ColorMode::Random
    };
    let alpha_mode = args.next().map(|alpha_mode| match alpha_mode.as_str() {
        "normal" => AlphaMode::Normal,
        "inverse" => AlphaMode::Inverse,
        _ => panic!("unknown alpha mode {}", alpha_mode)
    }).unwrap_or(AlphaMode::Normal);
 
    let mut output = RgbImage::new(image_width, image_height);
    let tiles = [&base_tile,
                    &imageops::rotate90(&base_tile),
                    &imageops::rotate180(&base_tile),
                    &imageops::rotate270(&base_tile)];


    for index_y in 0..image_height / base_tile.height() {
        for index_x in 0..image_width / base_tile.width() {
            let rot: usize = random::<usize>() % 4;
            let col: LinSrgba = color_mode.pick_color().into();

            for (x, y, image::Rgba(px)) in tiles[rot].enumerate_pixels() {
                let tile_color = Srgba::new((px[0] as f32) / 255.0,
                                    (px[1] as f32) / 255.0,
                                    (px[2] as f32) / 255.0,
                                    (px[3] as f32) / 255.0);
                let out_color: Srgb = tile_color.into_linear().mix(&col, alpha_mode.calculate_mix_param(tile_color.alpha)).into();
                output.put_pixel(index_x*base_tile.width() + x, index_y*base_tile.height() + y,
                    image::Rgb([(out_color.red * 255.0) as u8, (out_color.green * 255.0) as u8, (out_color.blue * 255.0) as u8]));
            }
        }
    }

    output.save(output_path)?;

    Ok(())
}
