use std::collections::HashMap;
use std::io::Cursor;
use crate::mesh::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum VoxelType{
    Air,
    Solid
}

#[derive(Debug, Clone)]
pub struct VoxelChunk {
    data: HashMap<[i8; 3], VoxelType>,
    size: usize,
    pos: [i16;3]
}
impl VoxelChunk{
	pub fn new(size: usize, pos: [i16;3]) -> Self{
		Self{
			data: HashMap::<[i8; 3], VoxelType>::new(),
			size: size,
			pos: pos
		}
	}
	
	pub fn generate_mesh<'a>(&mut self, display: &glium::Display) -> Mesh3d<'a>{
		let mut vertexbuffer: Vec<Vertex> = vec![];
		
		let mut indexbuffer: Vec<u16> = vec![];
		
		for x in 0..self.size{
			for y in 0..self.size{
				for z in 0..self.size{
		            self.data.insert([x as i8,y as i8,z as i8], VoxelType::Solid);
		            // test.block[x][y][z] = VoxelType::Solid
		        }
		    }
		}
		println!("{:?}", self.data);
		for (key, value) in &self.data{
//					if self.get_voxel([x as i8,y as i8,z as i8]) == VoxelType::Air {
//						println!("stop");
//					}
			self.generate_voxel_mesh(*key, &mut vertexbuffer, &mut indexbuffer)
					
//					if self.get_voxel([x as i8,y as i8 + 1i8,z as i8]) == VoxelType::Air{
//						println!("top");
//						for u in 0..blockibuffer[0].len(){
//							indexbuffer.insert(indexbuffer.len(), blockibuffer[0][u] + vertexbuffer.len() as u16);
//						}
//						for u in 0..blockvbuffer[0].len(){
//							vertexbuffer.insert(vertexbuffer.len(),
//								Vertex{
//									position: [blockvbuffer[0][u].position[0] + x as f32, blockvbuffer[0][u].position[1] + y as f32, blockvbuffer[0][u].position[2] + z as f32],
//									.. blockvbuffer[0][u]
//								}
//							);
//							println!("{:?}",[blockvbuffer[0][u].position[0] + x as f32, blockvbuffer[0][u].position[1] + y as f32, blockvbuffer[0][u].position[2] + z as f32]);
//						}
//					}
//					if self.get_voxel([x as i8,y as i8,z as i8 + 1i8]) == VoxelType::Air{
//						println!("front");
//						for u in 0..blockibuffer[4].len(){
//							indexbuffer.insert(indexbuffer.len(), blockibuffer[4][u] + vertexbuffer.len() as u16);
//						}
//						for u in 0..blockvbuffer[4].len(){
//							vertexbuffer.insert(vertexbuffer.len(),
//								Vertex{
//									position: [blockvbuffer[4][u].position[0] + x as f32, blockvbuffer[4][u].position[1] + y as f32, blockvbuffer[4][u].position[2] + z as f32],
//									.. blockvbuffer[4][u]
//								}
//							);
//						}
//					}
//		             println!("top {:?}, {:?}; botton {:?}, {:?},front {:?}, {:?}; back {:?}, {:?}, left {:?} {:?}, right {:?} {:?}",
//		                self.get_voxel([x as i8,y as i8 + 1i8,z as i8]),
//		                [x as i8,y as i8 + 1i8,z as i8],

//		                self.get_voxel([x as i8,y as i8 - 1i8,z as i8]),
//		                [x as i8,y as i8 - 1i8,z as i8],

//		                self.get_voxel([x as i8,y as i8,z as i8 + 1i8]),
//		                [x as i8,y as i8,z as i8 + 1i8],

//		                self.get_voxel([x as i8,y as i8,z as i8 - 1i8]),
//		                [x as i8,y as i8,z as i8 - 1i8],

//		                self.get_voxel([x as i8 - 1i8,y as i8,z as i8]),
//		                [x as i8 - 1i8,y as i8,z as i8],

//		                self.get_voxel([x as i8 + 1i8,y as i8,z as i8]),
//		                [x as i8 + 1i8,y as i8,z as i8],
//		             );
		}
//		println!("{:?}", indexbuffer);
		let image = image::load(Cursor::new(&include_bytes!("../textures/voxel.png")),
		                        image::ImageFormat::Png).unwrap().to_rgba8();
		let image_dimensions = image.dimensions();
		let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
		let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
		
		let vertex_shader_src = r#"
		    #version 150
		    
		    in vec3 position;
		    in vec3 normal;
		    in vec2 tex_coords;
		    
		    out vec3 v_normal;
		    out vec2 v_tex_coords;
		    
		    uniform mat4 perspective;
		    uniform mat4 view;
		    uniform mat4 model;
		    
		    void main() {
		        mat4 modelview = view * model;
		        
		        v_normal = transpose(inverse(mat3(modelview))) * normal;
		        v_tex_coords = tex_coords;
		        
		        gl_Position = perspective * modelview * vec4(position, 1.0);
		    }
		"#;
		let fragment_shader_src = r#"
		    #version 150
		    
		    in vec3 v_normal;
		    in vec2 v_tex_coords;
		    
		    out vec4 color;
		    
		    uniform vec3 u_light;
		    uniform sampler2D tex;
		    
		    void main() {
		        float brightness = dot(normalize(v_normal), normalize(u_light));
		        vec3 dark_color = vec3(0.6, 0.6, 0.6);
		        vec3 regular_color = vec3(1.0, 1.0, 1.0);
		        vec4 shadow = vec4(mix(dark_color, regular_color, brightness), 1.0);
				color = texture(tex, v_tex_coords) * shadow;
		    }
		"#;

		let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src,
		                                          None).unwrap();
			
		return Mesh3d::new(
			glium::VertexBuffer::new(display, &vertexbuffer.as_slice()).unwrap(),
			glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indexbuffer.as_slice()).unwrap(),
			[
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 2.0, 1.0f32]
			],
			Material{
				texture: texture,
				behavior: glium::uniforms::SamplerBehavior {
					minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
					magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
					.. Default::default()
				},
				shaders: program,
				draw_parameters: glium::DrawParameters {
				    depth: glium::Depth {
				        test: glium::draw_parameters::DepthTest::IfLess,
				        write: true,
				        .. Default::default()
				    },
		//			backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
				    .. Default::default()
				}
			}
		);
	}

	fn get_voxel(&self, pos: [i8;3]) -> VoxelType{
//		println!("{:?}", pos);
//		if pos[0] <= self.size as i8 && pos[0] >= 0i8 &&
//		pos[1] <= self.size as i8 && pos[1] >= 0i8 &&
//		pos[2] <= self.size as i8 && pos[2] >= 0i8 {
		match self.data.get(&pos) {
			Some(v) => return *v,
			None => return VoxelType::Air
		}
//		} else {
//			return VoxelType::Air;
//		}
	}
	
	fn generate_voxel_mesh(&self, pos: [i8;3], vertexbuffer: &mut Vec<Vertex>, indexbuffer: &mut Vec<u16>){
		
		let blockvbuffer = [// TODO: remove all `+ pos[] as f32`
			[
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },//top 4
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] }, 
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },
			],
			[
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },//botton 0
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },		
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
			],
			[
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },//left 8
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] },
			],
			[
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },//right 12
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] },
			],
			[
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },//front 16
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.5] },
			],
			[
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },//back 20
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 0.0, -1.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 0.0, -1.0], tex_coords: [0.3, 0.5] },
			]
		];
					
		let blockibuffer = [
			[
				2u16, 1, 0,//top
				2, 3, 1,
			],
			[
				0, 1, 2,   //botton
				1, 3, 2,
			],
			[
				0, 1, 2,  //left
				1, 3, 2,
			],
			[
				2, 1, 0,//right
				2, 3, 1,
			],
			[
				0, 1, 2,//front
				1, 3, 2,
			],
			[
				2, 1, 0,//back
				2, 3, 1,
			], 
		];
		
		
		if self.get_voxel(pos) == VoxelType::Air {
			print!("stop");
			return;
		}
		
		if self.get_voxel([pos[0],pos[1] + 1i8,pos[2]]) == VoxelType::Air{// top
			print!("top");
			for u in 0..blockibuffer[0].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[0][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[0].len(){
				vertexbuffer.insert(vertexbuffer.len(),
					blockvbuffer[0][u]
				);
			}
		}
		
		if self.get_voxel([pos[0],pos[1] - 1i8,pos[2]]) == VoxelType::Air{// botton
			print!("botton");
			for u in 0..blockibuffer[1].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[1][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[1].len(){
				vertexbuffer.insert(vertexbuffer.len(),
					blockvbuffer[0][u]
				);
			}
		}
		
		if self.get_voxel([pos[0] - 1i8,pos[1],pos[2]]) == VoxelType::Air{// left
			print!("left");
			for u in 0..blockibuffer[2].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[2][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[2].len(){
				vertexbuffer.insert(vertexbuffer.len(),
					blockvbuffer[0][u]
				);
			}
		}
		
		if self.get_voxel([pos[0] - 1i8,pos[1],pos[2]]) == VoxelType::Air{// right
			print!("right");
			for u in 0..blockibuffer[3].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[3][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[3].len(){
				vertexbuffer.insert(vertexbuffer.len(),
					blockvbuffer[0][u]
				);
			}
		}
		
		if self.get_voxel([pos[0],pos[1],pos[2] + 1i8]) == VoxelType::Air{// front
			print!("front");
			for u in 0..blockibuffer[4].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[4][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[4].len(){
				vertexbuffer.insert(vertexbuffer.len(),
					blockvbuffer[0][u]
				);
			}
		}
		
		if self.get_voxel([pos[0],pos[1],pos[2] - 1i8]) == VoxelType::Air{// back
			print!("back");
			for u in 0..blockibuffer[5].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[5][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[5].len(){
				vertexbuffer.insert(vertexbuffer.len(),
					blockvbuffer[0][u]
				);
			}
		}
	}
}
