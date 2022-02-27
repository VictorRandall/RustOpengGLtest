	let vertex_shader_src = r#"
		#version 140
		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;
		uniform mat4 matrix;
		void main() {
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
	"#;
	
	
	
use std::collections::HashMap;
use std::io::Cursor;
use crate::mesh::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum VoxelType{// o tipo de voxel
    Air,// significa q tem nada
    Solid// significa q tem um bloco
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Quad{// as faces do bloco
    Front([usize;3]),//frente
    Back([usize;3]),//atras
    Top([usize;3]),//cima
    Botton([usize;3]),//baixo
    Left([usize;3]),//esquerda
    Right([usize;3]),//direita
}

#[derive(Debug)]
pub struct VoxelChunk{
    pub pos: [i16;3],
    data: HashMap<[i8;3], VoxelType>,
    pub size: usize,
}

impl VoxelChunk{
    pub fn new(size: usize, pos: [i16;3]) -> Self{
        let mut chunk = VoxelChunk{
            pos: pos,
            data: HashMap::<[i8;3], VoxelType>::new(),
            size: size
        };

        for x in 0..chunk.size{
			for y in 0..chunk.size{
				for z in 0..chunk.size{
		            chunk.data.insert([x as i8,y as i8,z as i8], VoxelType::Solid);
		        }
		    }
		}

        return chunk;
    }
    pub fn generate_mesh<'a>(&self, display: &glium::Display) -> Mesh3d<'a>{
        let mut qbuffer: Vec<Quad> = vec![];

        for (key, value) in &self.data{
            if self.get_voxel([key[0], key[1] + 1i8, key[2]]) == VoxelType::Air{
                qbuffer.insert(
                    qbuffer.len(),
                    Quad::Top([key[0] as usize, key[1] as usize, key[2] as usize])
                );
            }//top
            if self.get_voxel([key[0], key[1] - 1i8, key[2]]) == VoxelType::Air{
                qbuffer.insert(
                    qbuffer.len(),
                    Quad::Botton([key[0] as usize, key[1] as usize, key[2] as usize])
                );
            }//botton
            if self.get_voxel([key[0] + 1i8, key[1], key[2]]) == VoxelType::Air{
                qbuffer.insert(
                    qbuffer.len(),
                    Quad::Left([key[0] as usize, key[1] as usize, key[2] as usize])
                );
            }//left
            if self.get_voxel([key[0] - 1i8, key[1], key[2]]) == VoxelType::Air{
                qbuffer.insert(
                    qbuffer.len(),
                    Quad::Right([key[0] as usize, key[1] as usize, key[2] as usize])
                );
            }//right
            if self.get_voxel([key[0], key[1], key[2] + 1i8]) == VoxelType::Air{
                qbuffer.insert(
                    qbuffer.len(),
                    Quad::Front([key[0] as usize, key[1] as usize, key[2] as usize])
                );
            }//front
            if self.get_voxel([key[0], key[1], key[2] - 1i8]) == VoxelType::Air{
                qbuffer.insert(
                    qbuffer.len(),
                    Quad::Back([key[0] as usize, key[1] as usize, key[2] as usize])
                );
            }//back
        }
        println!("{:?}", qbuffer);

        let blockvbuffer = [
			[
				Vertex{ position: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },//top 4
				Vertex{ position: [1.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },//  
				Vertex{ position: [0.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },// 
				Vertex{ position: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },//
			],
			[
				Vertex{ position: [0.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },//botton 0
				Vertex{ position: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },//		
				Vertex{ position: [0.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },//
				Vertex{ position: [1.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },//
			],
			[
				Vertex{ position: [0.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },//left 8
				Vertex{ position: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [1.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] },
			],
			[
				Vertex{ position: [0.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },//right 12
				Vertex{ position: [1.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] },
			],
			[
				Vertex{ position: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },//front 16
				Vertex{ position: [1.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.5] },
			],
			[
				Vertex{ position: [0.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },//back 20
				Vertex{ position: [0.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.3, 0.0] },
				Vertex{ position: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.5] },
				Vertex{ position: [0.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.3, 0.5] },
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

        let mut vertexbuffer: Vec<Vertex> = vec![];
		
		let mut indexbuffer: Vec<u16> = vec![];

        for value in qbuffer{
            match value{
                Quad::Top(v) => {
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 1.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] });
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 1.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] });
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 1.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] });
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 1.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] });
                },
                Quad::Botton(v) => {
                	vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 0.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] });
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 0.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] });
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 0.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] });
                    vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 0.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] });
                },
                Quad::Left(v) => {
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 0.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] });
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 0.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] });
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 1.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] });
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 1.0 + (v[1] as f32), 0.0 + (v[2] as f32)], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] });
                },
                Quad::Right(v) => {
                	vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 0.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] });
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 0.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] });
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [0.0 + (v[0] as f32), 1.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] });
					vertexbuffer.insert(vertexbuffer.len(), Vertex{ position: [1.0 + (v[0] as f32), 1.0 + (v[1] as f32), 1.0 + (v[2] as f32)], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] });
                },
                Quad::Front(v) => {
                
                },
                Quad::Back(v) => {
                	
                },
            }
        }
        
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
        match self.data.get(&pos) {
            Some(v) => return *v,
            None => return VoxelType::Air
        }
    }
}

