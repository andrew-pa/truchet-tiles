















use std::error::Error;
use image::{RgbImage,RgbaImage};
use image::io::Reader;
use image::GenericImage;
use image::GenericImageView;
use image::imageops;
use rand::prelude::*;
use palette::{Srgba, Srgb, LinSrgba, LinSrgb, Hsva, Mix};

fn parse_rgb(s: String) -> Result<(u8,u8,u8), Box<dyn std::error::Error>> {
    let mut raw = s.split(',').map(|x| x.parse::<u8>());
    Ok((raw.next().ok_or("expected value for color")??,
        raw.next().ok_or("expected value for color")??,
        raw.next().ok_or("expected value for color")??))
}

// usage: wangtiles <base tile image> <width> <height> <outputname>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let base_tile = image::open(args.next().ok_or("expected tile image path")?)?.to_rgba();
    let image_width = args.next().ok_or("expected image width")?.parse::<u32>()?;
    let image_height = args.next().ok_or("expected image height")?.parse::<u32>()?;
    let output_path = args.next().ok_or("expected output path")?;
 
    let mut output = RgbImage::new(image_width, image_height);
    let mut scratch_tile = RgbaImage::new(base_tile.width(), base_tile.height());


    for index_y in 0..image_height / base_tile.height() {
        for index_x in 0..image_width / base_tile.width() {
            let rot: u8 = random::<u8>() % 4;
            let col: LinSrgba = Hsva::new(random::<f32>().powf(1.5) * 360.0, 0.9, random::<f32>()*0.3 + 0.7, 1.0).into();
            match rot {
                0 => imageops::replace(&mut scratch_tile, &base_tile, 0, 0),
                1 => imageops::rotate90_in(&base_tile, &mut scratch_tile)?,
                2 => imageops::rotate180_in(&base_tile, &mut scratch_tile)?,
                3 => imageops::rotate270_in(&base_tile, &mut scratch_tile)?,
                _ => unreachable!()
            }
            for (x, y, image::Rgba(px)) in scratch_tile.enumerate_pixels() {
                let tile_color = Srgba::new((px[0] as f32) / 255.0,
                                    (px[1] as f32) / 255.0,
                                    (px[2] as f32) / 255.0,
                                    (px[3] as f32) / 255.0);
                let out_color = tile_color.into_linear().mix(&col, tile_color.alpha);
                output.put_pixel(index_x*base_tile.width() + x, index_y*base_tile.height() + y,
                    image::Rgb([(out_color.red * 255.0) as u8, (out_color.green * 255.0) as u8, (out_color.blue * 255.0) as u8]));
            }
        }
    }

    output.save(output_path)?;

    Ok(())
}
