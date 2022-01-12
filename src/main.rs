use petpet::{encode_gif, generate};
use std::io::BufWriter;
use std::path::PathBuf;
use std::process::exit;

use nfd2::Response;
use nfd2::{open_file_dialog, open_save_dialog};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = open_file_dialog(Some("jpg,jpeg,png,ico,bmp,webp,tiff"), None)?;
    let output_file = open_save_dialog(Some("gif"), None)?;

    let input_file = match_response(input_file);
    let output_file = match_response(output_file);

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

fn match_response(r: Response) -> PathBuf {
    match r {
        Response::Okay(p) => p,
        Response::Cancel => exit(0),
        _ => panic!(),
    }
}
