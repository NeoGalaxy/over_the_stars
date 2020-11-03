extern crate sdl2;

use sdl2::pixels::Color;
#[derive(Debug)]
pub struct BlockType {
	pub name: &'static str,
	pub id: usize,
	pub hardness: Option<u32>,
	pub light_level: i32,
	pub is_solid: bool,
	pub can_fall: bool,
	pub texture: Option<Color>,
	//pub outline: Color,
}

const BLOCK_TYPES: [BlockType; 5] = [
	BlockType{name: "void",
		id: 0,
		hardness: None,
		light_level: -1,
		is_solid: false, can_fall: false,
		texture: None
	},
	BlockType{name: "stone",
		id: 1,
		hardness: Some(10),
		light_level: -4,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(136,136,136))
	},
	BlockType{name: "dirt",
		id: 2,
		hardness: Some(6),
		light_level: -4,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(170,85,51))
	},
	BlockType{name: "iron_ore",
		id: 3,
		hardness: Some(15),
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(187,187,187))
	},
	BlockType{name: "gold_ore",
		id: 4,
		hardness: Some(15),
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(187,170,51))
	},
];

impl Default for &BlockType {
	fn default() -> &'static BlockType {
		&BLOCK_TYPES[1]
	}
}

pub fn get(id: usize) -> &'static BlockType {
	&BLOCK_TYPES[id]
}

pub fn get_some(id: usize) -> Option<&'static BlockType> {
	match BLOCK_TYPES.get(id) {
		Some(block) => Some(&block),
		None => None
	}
}

