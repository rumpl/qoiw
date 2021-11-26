use std::{fs::File, io::BufWriter, num::NonZeroUsize, time::Duration};

// use image::GenericImageView;
use qoi::Pixel;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::TextureAccess,
};

fn main() {
    // let img = image::open("cat.jpg").unwrap();

    // let w = img.width();
    // let h = img.height();
    // let pixels = img.pixels();
    // let file = File::create("cat.qoi").unwrap();

    // let p = pixels.map(|(_, _, p)| Pixel::rgba(p.0[0], p.0[1], p.0[2], p.0[3]));

    // qoi::encode(
    //     NonZeroUsize::new(w as usize).unwrap(),
    //     NonZeroUsize::new(h as usize).unwrap(),
    //     p,
    //     BufWriter::new(file),
    // )
    // .unwrap();

    let mut pxls: Vec<Pixel> = Vec::new();
    let (width, height) = qoi::decode_file_into_vec("cat.qoi", &mut pxls).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Streaming, w, h)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        texture.with_lock(None, |a, _| cc(a, &pxls)).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn cc(a: &mut [u8], pxls: &Vec<Pixel>) {
    let u128_slice: &mut [u32] = bytemuck::cast_slice_mut(a);

    for (i, l) in u128_slice.iter_mut().enumerate() {
        *l = (pxls[i].r as u32) << 24
            | (pxls[i].g as u32) << 16
            | (pxls[i].b as u32) << 8
            | (pxls[i].a as u32) << 0;
    }
}
