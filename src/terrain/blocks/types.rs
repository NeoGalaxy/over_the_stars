extern crate sdl2;

use sdl2::pixels::Color;
#[derive(Debug)]
pub struct BlockType {
	pub name: &'static str,
	pub id: usize,
	pub hp: Option<u32>,
	pub hardness: u32,
	pub light_level: i32,
	pub is_solid: bool,
	pub can_fall: bool,
	pub texture: Option<Color>,
	pub outline_texture: Option<Color>,
	pub bg_texture: Option<Color>,
	pub bg_outline_texture: Option<Color>,
}

pub const BLOCK_TYPES: [BlockType; 8] = [
	BlockType{name: "void", id: 0,
		hp: None,
		hardness: 0,
		light_level: -1,
		is_solid: false, can_fall: false,
		texture: None,
		outline_texture: None,
		bg_texture: None,
		bg_outline_texture: None
	},
	BlockType{name: "stone", id: 1,
		hp: Some(10),
		hardness : 8,
		light_level: -6,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(136,136,136)),
		outline_texture: Some(Color::RGB(105,105,105)),
		bg_texture: Some(Color::RGB(126,126,126)),
		bg_outline_texture: Some(Color::RGB(105,105,105)),
	},
	BlockType{name: "dirt", id: 2,
		hp: Some(6), hardness : 2,
		light_level: -4,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(140,85,51)),
		outline_texture: Some(Color::RGB(125,70,36)),
		bg_texture: Some(Color::RGB(130,75,41)),
		bg_outline_texture: Some(Color::RGB(125,70,36)),
	},
	BlockType{name: "iron_ore", id: 3,
		hp: Some(15), hardness : 11,
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(187,187,187)),
		outline_texture: Some(Color::RGB(157,157,157)),
		bg_texture: Some(Color::RGB(187,187,187)),
		bg_outline_texture: Some(Color::RGB(187,187,187)),
	},
	BlockType{name: "gold_ore", id: 4,
		hp: Some(15), hardness : 15,
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(187,170,51)),
		outline_texture: Some(Color::RGB(157,150,41)),
		bg_texture: Some(Color::RGB(187,170,51)),
		bg_outline_texture: Some(Color::RGB(187,170,51)),
	},
	BlockType{name: "gold_ore", id: 4,
		hp: Some(15), hardness : 15,
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(255,255,255)),
		outline_texture: Some(Color::RGB(255,255,255)),
		bg_texture: Some(Color::RGB(255,255,255)),
		bg_outline_texture: Some(Color::RGB(255,255,255)),
	},
	BlockType{name: "gold_ore", id: 4,
		hp: Some(15), hardness : 15,
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(0,0,0)),
		outline_texture: Some(Color::RGB(0,0,0)),
		bg_texture: Some(Color::RGB(0,0,0)),
		bg_outline_texture: Some(Color::RGB(0,0,0)),
	},
		BlockType{name: "gold_ore", id: 4,
		hp: Some(15), hardness : 15,
		light_level: -2,
		is_solid: true, can_fall: false,
		texture: Some(Color::RGB(123,214,83)),
		outline_texture: Some(Color::RGB(123,214,83)),
		bg_texture: Some(Color::RGB(123,214,83)),
		bg_outline_texture: Some(Color::RGB(123,214,83)),
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

/*pub fn get_some(id: usize) -> Option<&'static BlockType> {
	match BLOCK_TYPES.get(id) {
		Some(block) => Some(&block),
		None => None
	}
}*/

