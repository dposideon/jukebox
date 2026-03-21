use std::path::PathBuf;

use image::Luma;
use qrcode::QrCode;

use crate::networking::STATIC_DIR;

pub fn create_qr_code(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = PathBuf::from(STATIC_DIR).join("qr.png");

    let qr_code = QrCode::new(address.as_bytes())?;
    let image = qr_code.render::<Luma<u8>>().build();

    image.save(output_path)?;

    Ok(())
}
