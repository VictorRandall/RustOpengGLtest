#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}

glium::implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub position: [f32; 2],
//    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}

glium::implement_vertex!(Point, position, tex_coords);

#[derive(Debug)]
pub struct Material<'a>{
	pub texture: glium::texture::SrgbTexture2d,
	pub behavior: glium::uniforms::SamplerBehavior,
	pub shaders: glium::Program,
	pub draw_parameters: glium::DrawParameters<'a>
}

#[derive(Debug)]
pub struct Mesh3d<'a>{// 3dmodel 
	pub v_buffer: glium::VertexBuffer<Vertex>,
	pub i_buffer: glium::IndexBuffer<u16>,
	pub model: [[f32; 4]; 4],
	pub material: Material<'a>
}

impl<'a> Mesh3d<'a>{
	pub fn new(
				v_buffer: glium::VertexBuffer<Vertex>, 
				i_buffer: glium::IndexBuffer<u16>, 
				model: [[f32; 4]; 4], 
				material: Material<'a>
			) -> Self{
		Self {
			v_buffer: v_buffer,
			i_buffer: i_buffer,
			model: model,
			material: material
		}
	}
}

#[derive(Debug)]
pub struct Sprite<'a>{// 2dsprite
	pub v_buffer: glium::VertexBuffer<Point>,
	pub i_buffer: glium::IndexBuffer<u16>,
	pub model: [[f32; 4]; 4],
	pub material: Material<'a>
}

impl<'a> Sprite<'a>{
	pub fn new(
				v_buffer: glium::VertexBuffer<Point>, 
				i_buffer: glium::IndexBuffer<u16>, 
				model: [[f32; 4]; 4], 
				material: Material<'a>
			) -> Self{
		Self {
			v_buffer: v_buffer,
			i_buffer: i_buffer,
			model: model,
			material: material
		}
	}
}
