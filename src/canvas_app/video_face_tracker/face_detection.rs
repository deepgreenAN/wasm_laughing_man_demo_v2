use rustface::{Detector, FaceInfo, ImageData};

pub fn detect_faces(
    detector: &mut dyn Detector,
    gray_vec: &Vec<u8>,
    width: u32,
    height: u32,
) -> Vec<FaceInfo> {
    let mut image = ImageData::new(gray_vec, width, height);
    let faces = detector.detect(&mut image);
    faces
}

/// r, g, b, a * width * heightの画像をluma * width * heightに変換
pub fn convert_rgba_to_luma(rgba_vec: Vec<u8>) -> Vec<u8> {
    rgba_vec
        .chunks_exact(4)
        .map(|rgba| {
            (0.299 * (rgba[0] as f64) + 0.587 * (rgba[1] as f64) + 0.114 * (rgba[2] as f64)) as u8
        })
        .collect::<Vec<_>>()
}
