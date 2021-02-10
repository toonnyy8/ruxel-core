use super::tui;
use image;

pub fn save(canvas: &mut tui::Canvas, name: String) {
    let size = canvas.size;
    let imgbuf = image::ImageBuffer::from_fn(size.x as u32, size.y as u32, |x, y| {
        let (r, g, b, a) = canvas.data[y as usize][x as usize].into();
        image::Rgba([r, g, b, a])
    });
    imgbuf.save(name).unwrap();
}
