use std::{env, error::Error};

use qoi::Pixel;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::TextureAccess,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let image = &args[1];
    let mut pxls: Vec<Pixel> = Vec::new();

    let (width, height) = qoi::decode_file_into_vec(image, &mut pxls)?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("QOI", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(
        PixelFormatEnum::RGBA8888,
        TextureAccess::Streaming,
        width as u32,
        height as u32,
    )?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.clear();

    texture.with_lock(None, |a, _| copy(a, &pxls))?;
    canvas.copy(&texture, None, None)?;
    canvas.present();

    'running: loop {
        let event = event_pump.wait_event();
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => break 'running,
            _ => {}
        }
    }

    Ok(())
}

fn copy(a: &mut [u8], pxls: &[Pixel]) {
    let u128_slice: &mut [u32] = bytemuck::cast_slice_mut(a);

    for (i, l) in u128_slice.iter_mut().enumerate() {
        *l = pxls[i].into();
    }
}
