extern crate sdl2; 
pub mod terrain;
pub mod inventory;
pub mod entities;

use std::thread::sleep;
use crate::Interaction::AimAt;
use crate::terrain::blocks::BLOCK_SIZE;
use std::collections::HashMap;
use std::io::Write;
use sdl2::{
	pixels::Color,
	event::Event,
	keyboard::Keycode,
	mouse::MouseButton,
};
use sdl2_sys::SDL_WindowFlags;
use vek::vec::repr_c::vec2::Vec2;

use std::time::{Duration, Instant};

use terrain::{
	map::{
		Map,
		Interaction
	}
};
use entities::{player::Player};

fn update_mouse(mouse: &Vec2<f64>, map: &mut Map) {
	let block_pos: Vec2<i32> = (mouse / BLOCK_SIZE as f64).floor().as_();
	map.interact(AimAt(block_pos), true);
}

fn main() {
	let tick_duration = Duration::from_secs_f32(1./60.);
	let mut last_tick = Instant::now();
	let key_mapping : HashMap<Keycode, Interaction> =
	[(Keycode::Z, Interaction::Up),
	 (Keycode::S, Interaction::Down),
	 (Keycode::Q, Interaction::Left),
	 (Keycode::D, Interaction::Right)]
	 .iter().cloned().collect();
	let mouse_mapping : HashMap<MouseButton, Interaction> =
	[(MouseButton::Left,  Interaction::Attack),
	 (MouseButton::Right, Interaction::Use)]
	 .iter().cloned().collect();
	let mut action_active : [bool; 4] = [false, false, false, false];
	let mut mouse_pos: Option<Vec2<f64>> = None;

	let mut map = Map::new();
	let player_id = Player::create(Vec2::new(0.,-10.0), &mut map);
	map.set_interacter(player_id);
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
 
	let window = video_subsystem.window("Above", 800, 600)
		.set_window_flags(SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32)
		.position_centered()
		.position_centered()
		.build()
		.unwrap();
	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
	let mut pos = Vec2::new(0.,0.);
	{
		//let pos = map.see_entity(player_id).get_pos();
		map.disp(&mut canvas, &pos);
	}
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut cnt = 0;
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					print!("\u{001b}[0K");
					std::io::stdout().flush().expect("Hmmm......");
					break 'running
				},
				Event::KeyDown { keycode: Some(code), .. } => {
					match code {
						Keycode::Up => {action_active[0] = true;}
						Keycode::Down => {action_active[1] = true;}
						Keycode::Left => {action_active[2] = true;}
						Keycode::Right => {action_active[3] = true;},
						_ => {
							if let Some(interaction) = key_mapping.get(&code) {
								map.interact(*interaction, true);
							}
						}
					}
				},
				Event::KeyUp { keycode: Some(code), .. } => {
					match code {
						Keycode::Up => {action_active[0] = false;}
						Keycode::Down => {action_active[1] = false;}
						Keycode::Left => {action_active[2] = false;}
						Keycode::Right => {action_active[3] = false;},
						_ => {
							if let Some(interaction) = key_mapping.get(&code) {
								map.interact(*interaction, false);
							}
						}
					}
				},
				Event::MouseMotion {x: mouse_x, y: mouse_y, ..} => {
					let (width, height) = canvas.output_size().unwrap();
					let screen_center = Vec2::new(width as i32, height as i32)/2;
					let px_camera = pos * BLOCK_SIZE as f64;
					mouse_pos = Some(Vec2::new(mouse_x, mouse_y).as_()
						- screen_center.as_() + px_camera);
					update_mouse(&mouse_pos.unwrap(), &mut map);
				},
				Event::MouseButtonDown {mouse_btn: button, ..} => {
					if let Some(interaction) = mouse_mapping.get(&button) {
						map.interact(*interaction, true);
					}
				},
				Event::MouseButtonUp {mouse_btn: button, ..} => {
					if let Some(interaction) = mouse_mapping.get(&button) {
						map.interact(*interaction, false);
					}
				}
				_ => {}
			}
		}
		cnt += 1;
		if cnt == 10 {
			//print!("{:3.4}", 10./intervall);
			//print!("{:3.4} {:3.4}", pos.x, pos.y);
			//print!("    ");
			//print!("\r");
			cnt = 0;
		}
		map.tick();
		let new_pos = map.see_entity(player_id).get_pos();
		if let Some(mpos) = mouse_pos {
			mouse_pos = Some(mpos + (new_pos - pos) * BLOCK_SIZE as f64);
			update_mouse(&mouse_pos.unwrap(), &mut map);
		}
		pos = new_pos;
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		map.disp(&mut canvas, &pos);
		canvas.present();

		let elapsed = last_tick.elapsed();
		if elapsed < tick_duration {
			sleep(tick_duration - elapsed);
		} else {
			println!("Too long: {:?}", elapsed - tick_duration);
		}
		last_tick = Instant::now();
	}
}
