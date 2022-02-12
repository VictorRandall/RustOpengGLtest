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
		self.yaw += input.mouse_motion[0] as f32 / 1000f32 * 3f32;
		self.pitch += input.mouse_motion[1] as f32 / 1000f32 * 3f32;
		
		if 0f32 > self.yaw{
			self.yaw = 0f32;
		}
		if -1f32 > self.pitch{
			self.pitch = -1f32;
		}
		
		if input.is_key_pressed(17u32) == true{
//			println!("walking foward");
			self.pos[0] += self.yaw.cos() * self.pitch.cos() * delta * 4f32;
			self.pos[1] += self.pitch.sin() * delta * 4f32;
			self.pos[2] += self.yaw.sin() * self.pitch.cos() * delta * 4f32;
		}else if input.is_key_pressed(31u32) == true{
//			println!("walking backward");
			self.pos[0] -= self.yaw.cos() * self.pitch.cos() * delta * 4f32;
			self.pos[1] -= self.pitch.sin() * delta * 4f32;
			self.pos[2] -= self.yaw.sin() * self.pitch.cos() * delta * 4f32;
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
