use crate::input::*;

#[derive(Debug)]
pub struct Camera{
	pub yaw: f32,
	pub pitch: f32,
	pub pos: [f32;3],
	pub view_matrix: [[f32; 4]; 4]
}

impl Camera {
	pub fn new() -> Self{
		Self{
			yaw: 0f32,
			pitch: 0f32,
			pos: [0f32,0f32,0f32],
			view_matrix: [[0f32;4usize];4usize]
		}
	}
	pub fn update(&mut self, delta: f32, input: &InputHandler){
		let mouse_sensitivity = 80f32;
		let cam_speed = 4f32;
		
		let front = [
			self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			self.pitch.to_radians().sin(), 
			self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
		];
		
		let up = [0f32,1f32,0f32];
		
		let left = [
			(front[1] * up[2]) - (front[2] * up[1]),
			(front[2] * up[0]) - (front[0] * up[2]),
			(front[0] * up[1]) - (front[1] * up[0])
		];
		
		self.yaw += input.mouse_motion[0] as f32 * delta * mouse_sensitivity;
		self.pitch += input.mouse_motion[1] as f32 * delta * mouse_sensitivity;
		
		if 0f32 < self.yaw{
			self.yaw = 0f32;
		}
		if -90f32 > self.pitch{
			self.pitch = -90f32;
		}
		
		if input.is_key_pressed(17u32) == true{
//			println!("walking fowards");
			self.pos[0] += front[0] * delta * cam_speed;
			self.pos[1] += front[1] * delta * cam_speed;
			self.pos[2] += front[2] * delta * cam_speed;
		}else if input.is_key_pressed(31u32) == true{
//			println!("walking backwards");
			self.pos[0] -= front[0] * delta * cam_speed;
			self.pos[1] -= front[1] * delta * cam_speed;
			self.pos[2] -= front[2] * delta * cam_speed;
		}
		
		if input.is_key_pressed(30u32) == true{
//			println!("walking left");
			self.pos[0] += left[0] * delta * cam_speed;
			self.pos[1] += left[1] * delta * cam_speed;
			self.pos[2] += left[2] * delta * cam_speed;
		}else if input.is_key_pressed(32u32) == true{
//			println!("walking right");
			self.pos[0] -= left[0] * delta * cam_speed;
			self.pos[1] -= left[1] * delta * cam_speed;
			self.pos[2] -= left[2] * delta * cam_speed;
		}
//		println!("{} {}", self.yaw, self.pitch);
//		if self.yaw > 89
	}
}
