use screenshot::Screen;
use image::{ImageBuffer, Rgba};

fn run() -> Result<(), Box<dyn Error>> {
    let screen = Screen::all()?;

    let display = screen.first().ok_or("No screens found")?;

    let image = display.capture()?;

    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(image.width(), image.height(), image.into_raw())
            .ok_or("Failed to create image buffer")?;

    buffer.save("screenshot.png")?;
    
    Ok(())
}
