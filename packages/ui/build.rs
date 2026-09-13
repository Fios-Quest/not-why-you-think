use qrcode_generator::qr::{Encoder, ErrorCorrection};
use qrcode_generator::Renderer;
use std::error::Error;
use std::fs::exists;

fn main() -> Result<(), Box<dyn Error>> {
    // Create the Tris Video QR code if it doesn't exist
    let tris_video_qr_file = "assets/rust-is-easy-link.png";
    if !exists(tris_video_qr_file)? {
        let symbol = Encoder::new(ErrorCorrection::Medium)
            .encode_text("https://www.youtube.com/watch?v=CJtvnepMVAU")?;

        Renderer::new(&symbol, 512)
            .quiet_zone(1)
            .save_png(tris_video_qr_file)?;
    }

    Ok(())
}
