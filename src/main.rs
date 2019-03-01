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

    #[structopt(short = "s", long = "scale", default_value = "1.0")]
    scale: f32,

    #[structopt(
        short = "o",
        long = "output",
        default_value = "output.png",
        parse(from_os_str)
    )]
    output: PathBuf,

    #[structopt(name = "IMAGE", parse(from_os_str))]
    images: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if opt.images.is_empty() {
        eprintln!("Error: at least one image must be specified");
        process::exit(1);
    }

    // Compute grid dimensions
    let other = |x: u32| {
        let len = opt.images.len() as u32;
        len / x + if len % x != 0 { 1 } else { 0 }
    };
    let (rows, cols) = match (opt.rows, opt.columns) {
        (0, 0) => {
            let columns = (opt.images.len() as f32).sqrt().ceil() as u32;
            (other(columns), columns)
        }
        (a, 0) => (a, other(a)),
        (0, b) => (other(b), b),
        (a, b) => {
            if a * b < opt.images.len() as u32 {
                eprintln!("Error: output grid not large enough");
                process::exit(1);
            }
            (a, b)
        }
    };
    println!("grid: rows={} cols={}", rows, cols);

    let mut images: Vec<image::DynamicImage> = vec![];
    for image in opt.images {
        images.push(image::open(image)?);
    }

    let max_width = images.iter().map(|i| i.width()).max().unwrap();
    let max_height = images.iter().map(|i| i.height()).max().unwrap();

    let mut output = image::ImageBuffer::new(max_width * cols, max_height * rows);
    for (i, image) in images.iter().enumerate() {
        let i = i as u32;
        let dx = (i % cols) * max_width;
        let dy = (i / cols) * max_height;
        println!("Copying image: {}, to: ({},{})", i, dx, dy);
        output.copy_from(image, dx, dy);
    }

    if let Err(e) = output.save(&opt.output) {
        eprintln!("Error saving image: {}", e);
        process::exit(1);
    }

    Ok(())
}
