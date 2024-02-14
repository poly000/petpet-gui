use petpet::{encode_apng, encode_gif, generate};
use std::io::BufWriter;
use std::process;

use rfd::FileDialog;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_file = FileDialog::new()
        .add_filter(
            "Origin Image",
            &["jpg", "jpeg", "png", "ico", "bmp", "webp", "tiff"],
        )
        .pick_file()
        .unwrap_or_else(|| process::exit(0));

    let output_file = FileDialog::new()
        .add_filter("", &["gif", "png"])
        .save_file()
        .unwrap_or_else(|| process::exit(0));

    let output = std::fs::File::create(&output_file)?;

    let frames = generate(
        petpet::image::open(input_file)?.to_rgba8(),
        petpet::FilterType::Lanczos3,
        None,
    )?;

    if output_file
        .file_name()
        .map(|file_name| file_name.to_string_lossy())
        .is_some_and(|file_name| file_name.ends_with(".png"))
    {
        let delay_num = 1;
        let delay_den = 50;
        encode_apng(
            frames,
            output,
            petpet::png::FilterType::NoFilter,
            delay_num,
            delay_den,
        )?;
    } else {
        encode_gif(frames, BufWriter::new(output), 1)?;
    }

    Ok(())
}
