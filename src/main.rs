extern crate sdl2; 
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod terrain;
use terrain::{chunks::ChunkMap};

fn main() {
	let mut map = ChunkMap::new();
	let chunk1 = map.get(0,0);
	let chunk2 = map.get(0,1);

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
 
	let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
		.position_centered()
		.build()
		.unwrap();
	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	chunk1.disp(&mut canvas, 2,2);
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
