use std::{error::Error, path::PathBuf, process};

use image::{GenericImage, GenericImageView};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "tile-image")]
pub struct Opt {
    #[structopt(short = "r", long = "rows", default_value = "0")]
    rows: u32,

    #[structopt(short = "c", long = "columns", default_value = "0")]
    columns: u32,

    #[structopt(long)]
    center: bool,

    #[structopt(long = "center-vertical")]
    center_vertical: bool,

    #[structopt(long = "center-horizontal")]
    center_horizontal: bool,

    #[structopt(short = "s", long = "scale", default_value = "1.0")]
    scale: f32,

    #[structopt(short = "o", long = "output", default_value = "output.png", parse(from_os_str))]
    output: PathBuf,

    #[structopt(name = "IMAGE", parse(from_os_str))]
    images: Vec<PathBuf>,

    #[structopt(short = "p", long = "pattern")]
    pattern: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut files = opt.images;

    if let Some(pattern) = opt.pattern {
        let paths = match glob::glob(&pattern) {
            Ok(paths) => paths,
            Err(e) => {
                eprintln!("Error: invalid pattern: {}", e);
                process::exit(1);
            }
        };

        for path in paths {
            if let Ok(path) = path {
                if path.is_file() {
                    files.push(path);
                }
            }
        }
    }

    if files.is_empty() {
        eprintln!("Error: at least one image must be specified");
        process::exit(1);
    }

    // Compute grid dimensions
    let other = |x: u32| {
        let len = files.len() as u32;
        len / x + if len % x != 0 { 1 } else { 0 }
    };
    let (rows, cols) = match (opt.rows, opt.columns) {
        (0, 0) => {
            let columns = (files.len() as f32).sqrt().ceil() as u32;
            (other(columns), columns)
        }
        (a, 0) => (a, other(a)),
        (0, b) => (other(b), b),
        (a, b) => {
            if a * b < files.len() as u32 {
                eprintln!("Error: output grid not large enough");
                process::exit(1);
            }
            (a, b)
        }
    };
    println!("grid: rows={} cols={}", rows, cols);

    let mut images: Vec<image::DynamicImage> = vec![];
    for path in files {
        images.push(image::open(path)?);
    }

    let max_width = images.iter().map(|i| i.width()).max().unwrap();
    let max_height = images.iter().map(|i| i.height()).max().unwrap();

    let mut output = image::ImageBuffer::new(max_width * cols, max_height * rows);
    for (i, image) in images.iter().enumerate() {
        let i = i as u32;

        let mut dx = (i % cols) * max_width;
        if opt.center || opt.center_horizontal {
            dx += (max_width - image.width()) / 2;
        }

        let mut dy = (i / cols) * max_height;
        if opt.center || opt.center_vertical {
            dy += (max_height - image.height()) / 2;
        }

        println!("Copying image: {}, to: ({},{})", i, dx, dy);
        output.copy_from(image, dx, dy);
    }

    if let Err(e) = output.save(&opt.output) {
        eprintln!("Error saving image: {}", e);
        process::exit(1);
    }

    Ok(())
}
