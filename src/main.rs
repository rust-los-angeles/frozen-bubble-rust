extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("Frozen-Bubble 2", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let icon = Surface::load_bmp("./share/gfx/pinguins/window_icon_penguin.bmp").unwrap();
    window.set_icon(icon);

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}