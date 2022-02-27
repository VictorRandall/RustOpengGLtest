use std::collections::HashMap;
use glium::{glutin, Surface};

#[derive(Debug)]
pub struct InputHandler{
	keys: HashMap::<u32, bool>,
	pub mouse_motion: [f64; 2],
	pub mouse_pos: [f64; 2]
}

impl InputHandler{
	pub fn new() -> Self{
		Self {
			keys: HashMap::<u32, bool>::new(),
			mouse_motion: [0f64, 0f64],
			mouse_pos: [0f64, 0f64]
		}
	}
	
	pub fn update_keyboard(&mut self, key: u32, state: glutin::event::ElementState) {
		let mut key_state = false;
		
		match state {
			glutin::event::ElementState::Pressed => key_state = true,
			glutin::event::ElementState::Released => key_state = false
		}
		
		self.keys.insert(key, key_state);
	}
	pub fn update_mouse(&mut self, mouse_pos: [f64;2]) {
		self.mouse_motion = [self.mouse_pos[0] - mouse_pos[0], self.mouse_pos[1] - mouse_pos[1]];
		
		self.mouse_pos = mouse_pos;
	}
	
	pub fn is_key_pressed(&self, key: u32) -> bool{
		match self.keys.get(&key) { //TODO if some then return bool, if none then return false
			Some(v) => return *v,
			None => return false
		}
	}
}
