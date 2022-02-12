#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}

glium::implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Debug)]
pub struct Mesh {
	pub v_buffer: glium::VertexBuffer<Vertex>,
	pub i_buffer: glium::IndexBuffer<u16>,
	pub model: [[f32; 4]; 4]
}

impl Mesh{
	pub fn new(v_buffer: glium::VertexBuffer<Vertex>, i_buffer: glium::IndexBuffer<u16>, model: [[f32; 4]; 4]) -> Self{
		Self {
			v_buffer: v_buffer,
			i_buffer: i_buffer,
			model: model
		}
	}
}
