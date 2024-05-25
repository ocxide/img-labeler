use std::{fs, io::Cursor, path::PathBuf};

use clap::Parser;
use image::{GenericImageView, Rgba};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the image
    #[clap(long)]
    pub source: PathBuf,

    /// Path to the output image file
    #[clap(long, short = 'o')]
    pub output: PathBuf,

    /// Text to add to the bottom of the image
    #[clap(long, short = 't')]
    pub title: String,

    /// Path to the font
    #[clap(long, short = 'f')]
    pub font: Option<PathBuf>,
}

fn main() {
    let Cli {
        source,
        title,
        output,
        font,
    } = Cli::parse();

    let filebuf = fs::read(source).expect("to read file");

    let font_src = font
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Bitter-Regular.ttf"));
    let font = fs::read(font_src).expect("to read font");
    let font = ab_glyph::FontVec::try_from_vec(font).expect("to be valid font");

    let orignal_pic =
        image::load(Cursor::new(&filebuf), image::ImageFormat::Png).expect("to load image");

    let (original_width, orignal_height) = orignal_pic.dimensions();

    let scale = ab_glyph::PxScale { x: 48.0, y: 48.0 };
    let (text_width, text_height) = imageproc::drawing::text_size(scale, &font, title.as_str());

    const Y_PADDING: u32 = 6;

    let text_box_y = orignal_height + text_height + (2 * Y_PADDING);
    let mut img = image::ImageBuffer::from_pixel(original_width, text_box_y, Rgba([0, 0, 0, 0]));

    let pic = orignal_pic.to_rgba8();
    image::imageops::overlay(&mut img, &pic, 0, 0);

    {
        use imageproc::point::Point;
        let color = image::Rgba([88, 88, 88, 255]);
        imageproc::drawing::draw_polygon_mut(
            &mut img,
            &[
                Point {
                    x: 0,
                    y: orignal_height as i32,
                },
                Point {
                    x: 0,
                    y: text_box_y as i32,
                },
                Point {
                    x: original_width as i32,
                    y: text_box_y as i32,
                },
                Point {
                    x: original_width as i32,
                    y: orignal_height as i32,
                },
            ],
            color,
        )
    };

    {
        let text_pos_w = (original_width / 2 - text_width / 2) as i32;
        let text_pos_h = (orignal_height + 0) as i32;

        imageproc::drawing::draw_text_mut(
            &mut img,
            image::Rgba([255, 255, 255, 255]),
            text_pos_w,
            text_pos_h,
            scale,
            &font,
            title.as_str(),
        );
    };

    img.save(output).expect("to save");
}
