use super::tui;
use image;

pub fn save(canvas: &mut tui::Canvas, name: String) {
    let size = canvas.size;
    let imgbuf = image::ImageBuffer::from_fn(size.x as u32, size.y as u32, |x, y| {
        image::Rgba(canvas.data[y as usize][x as usize].into())
    });
    imgbuf.save(name).unwrap();
}
