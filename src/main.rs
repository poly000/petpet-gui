use petpet::{encode_gif, generate};
use std::io::BufWriter;
use std::process;

use rfd::FileDialog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = FileDialog::new()
        .add_filter(
            "Origin Image",
            &["jpg", "jpeg", "png", "ico", "bmp", "webp", "tiff"],
        )
        .pick_file()
        .unwrap_or_else(|| process::exit(0));

    let output_file = FileDialog::new()
        .add_filter("", &["gif"])
        .save_file()
        .unwrap_or_else(|| process::exit(0));

    let output = std::fs::File::create(output_file)?;

    encode_gif(
        generate(
            petpet::image::open(input_file)?.to_rgba8(),
            petpet::FilterType::Lanczos3,
        )?,
        BufWriter::new(output),
        1,
    )?;

    Ok(())
}
