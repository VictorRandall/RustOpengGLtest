#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}

glium::implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Debug)]
pub struct Mesh3d {// modelo 3d
	pub v_buffer: glium::VertexBuffer<Vertex>,
	pub i_buffer: glium::IndexBuffer<u16>,
	pub model: [[f32; 4]; 4],
	pub texture: glium::texture::SrgbTexture2d,
	pub behavior: glium::uniforms::SamplerBehavior
}

impl Mesh3d{
	pub fn new(
				v_buffer: glium::VertexBuffer<Vertex>, 
				i_buffer: glium::IndexBuffer<u16>, 
				model: [[f32; 4]; 4], 
				texture: glium::texture::SrgbTexture2d, 
				behavior: glium::uniforms::SamplerBehavior
			) -> Self{
		Self {
			v_buffer: v_buffer,
			i_buffer: i_buffer,
			model: model,
			texture: texture,
			behavior: behavior
		}
	}
}

#[derive(Debug)]
pub struct Sprite {// um sprite 2d
	pub v_buffer: glium::VertexBuffer<Vertex>,
	pub i_buffer: glium::IndexBuffer<u16>,
	pub model: [[f32; 4]; 4],
	pub texture: glium::texture::SrgbTexture2d,
	pub behavior: glium::uniforms::SamplerBehavior,
	pub shaders: glium::Program
}

impl Sprite{
	pub fn new(
				v_buffer: glium::VertexBuffer<Vertex>, 
				i_buffer: glium::IndexBuffer<u16>, 
				model: [[f32; 4]; 4], 
				texture: glium::texture::SrgbTexture2d, 
				behavior: glium::uniforms::SamplerBehavior,
				shaders: glium::Program
			) -> Self{
		Self {
			v_buffer: v_buffer,
			i_buffer: i_buffer,
			model: model,
			texture: texture,
			behavior: behavior,
			shaders: 
		}
	}
}
