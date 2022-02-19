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
			self.pos[0] += self.yaw.to_radians().cos() * self.pitch.to_radians().cos() * delta * cam_speed;
			self.pos[1] += self.pitch.to_radians().sin() * delta * cam_speed;
			self.pos[2] += self.yaw.to_radians().sin() * self.pitch.to_radians().cos() * delta * cam_speed;
		}else if input.is_key_pressed(31u32) == true{
//			println!("walking backwards");
			self.pos[0] -= self.yaw.to_radians().cos() * self.pitch.to_radians().cos() * delta * cam_speed;
			self.pos[1] -= self.pitch.to_radians().sin() * delta * cam_speed;
			self.pos[2] -= self.yaw.to_radians().sin() * self.pitch.to_radians().cos() * delta * cam_speed;
		}
//		println!("{} {}", self.yaw, self.pitch);
//		if self.yaw > 89
	}
}
//TODO: testar isso dps
//void processInput(GLFWwindow *window)
//{
//    ...
//    const float cameraSpeed = 0.05f; // adjust accordingly
//    if (glfwGetKey(window, GLFW_KEY_W) == GLFW_PRESS)
//        cameraPos += cameraSpeed * cameraFront;
//    if (glfwGetKey(window, GLFW_KEY_S) == GLFW_PRESS)
//        cameraPos -= cameraSpeed * cameraFront;
//    if (glfwGetKey(window, GLFW_KEY_A) == GLFW_PRESS)
//        cameraPos -= glm::normalize(glm::cross(cameraFront, cameraUp)) * cameraSpeed;
//    if (glfwGetKey(window, GLFW_KEY_D) == GLFW_PRESS)
//        cameraPos += glm::normalize(glm::cross(cameraFront, cameraUp)) * cameraSpeed;
//}
