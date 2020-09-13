extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use sdl2::image::{InitFlag, LoadTexture};

use std::thread::sleep;
use std::time::{Duration, SystemTime};

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Black,
    //White,
    //Red,
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Black => texture.set_draw_color(Color::RGB(0, 0, 0)),
                    //TextureColor::White => texture.set_draw_color(Color::RGB(255, 255, 255)),
                    //TextureColor::Red => texture.set_draw_color(Color::RGB(255, 0, 0)),
                    TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");

    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect(
        "Couldn't initialize
         image context",
    );

    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let image_texture = texture_creator
        .load_texture("assets/my_image.png")
        .expect("Couldn't load image");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    let green_texture = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");
    let blue_texture = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");
    let black_texture = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Black,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");

    let now = SystemTime::now();
    'running: loop {
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

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas
            .copy(&image_texture, None, None)
            .expect("Render failed");

        let square_texture = if let Ok(elapsed) = now.elapsed() {
            //println!("elapsed={}", elapsed.as_secs());
            if elapsed.as_secs() % 2 == 0 {
                &green_texture
            } else {
                &blue_texture
            }
        } else {
            &black_texture
        };
        canvas
            .copy(
                square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("Couldn't copy texture into window");

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
