use std::collections::HashMap;
use crate::mesh::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum VoxelType{
    Air,
    Solid
}

#[derive(Debug, Clone)]
pub struct VoxelChunk {
    data: HashMap<[usize; 3], VoxelType>,
    size: usize,
    pos: [i16;3]
    // block: Vec<Vec<Vec<VoxelType>>>
}

impl VoxelChunk{
	pub fn new(size: usize, pos: [i16;3]) -> Self{
		Self{
			data: HashMap::<[usize; 3], VoxelType>::new(),
			size: size,
			pos: pos
		}
	}
	
	pub fn generate_mesh(&mut self, display: &glium::Display) -> Mesh{
		let mut vertexbuffer: Vec<Vertex> = vec![];
		
		let mut indexbuffer: Vec<u16> = vec![];
	
		for x in 0..self.size{
			for y in 0..self.size{
				for z in 0..self.size{
		            self.data.insert([x,y,z], VoxelType::Solid);
		            // test.block[x][y][z] = VoxelType::Solid
		        }
		    }
		}
		for x in 0..self.size{
			for y in 0..self.size{
				for z in 0..self.size{
//		             println!("top {:?}, {:?}; botton {:?}, {:?},front {:?}, {:?}; back {:?}, {:?}",
//		                self.get_voxel([x as i8,y as i8 + 1i8,z as i8]),
//		                [x as i8,y as i8 + 1i8,z as i8],
//		                self.get_voxel([x as i8,y as i8 - 1i8,z as i8]),
//		                [x as i8,y as i8 - 1i8,z as i8],
//		                self.get_voxel([x as i8,y as i8,z as i8 + 1i8]),
//		                [x as i8,y as i8,z as i8 + 1i8],
//		                self.get_voxel([x as i8,y as i8,z as i8 - 1i8]),
//		                [x as i8,y as i8,z as i8 - 1i8],
//		             );
					self.generate_voxel_mesh([x as i8,y as i8,z as i8], &mut vertexbuffer, &mut indexbuffer)
		        }
		    }
		}
		
		return Mesh::new(
			glium::VertexBuffer::new(display, &vertexbuffer.as_slice()).unwrap(),
			glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indexbuffer.as_slice()).unwrap(),
			[
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 2.0, 1.0f32]
			]
		);
	}

	fn get_voxel(&self, pos: [i8;3]) -> VoxelType{
		println!("{:?}", pos);
		if pos[0] < self.size as i8 && pos[0] > 0i8 &&
		pos[1] < self.size as i8 && pos[1] > 0i8 &&
		pos[2] < self.size as i8 && pos[2] > 0i8 {
		   match self.data.get(&[pos[0] as usize,pos[1] as usize,pos[2] as usize]) {
		      Some(v) => return *v,
		      None => return VoxelType::Air
		   }
		} else {
			return VoxelType::Air;
		}
	}
	
	fn generate_voxel_mesh(&self, pos: [i8;3], vertexbuffer: &mut Vec<Vertex>, indexbuffer: &mut Vec<u16>){
		let blockvbuffer = [// TODO: remove all `+ pos[] as f32`
			[
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },//top 4
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },//  
				Vertex{ position: [0.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },// 
				Vertex{ position: [1.0 + pos[0] as f32, 1.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },//
			],
			[
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },//botton 0
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 0.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },//		
				Vertex{ position: [0.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },//
				Vertex{ position: [1.0 + pos[0] as f32, 0.0 + pos[1] as f32, 1.0 + pos[2] as f32], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },//
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
					Vertex{ position: blockvbuffer[0][u].position,
					.. blockvbuffer[0][u]
					}
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
					Vertex{ position: blockvbuffer[1][u].position,
					.. blockvbuffer[1][u]
					}
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
					Vertex{ position: blockvbuffer[2][u].position,
					.. blockvbuffer[2][u]
					}
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
					Vertex{ position: blockvbuffer[3][u].position,
					.. blockvbuffer[3][u]
					}
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
					Vertex{ position: blockvbuffer[4][u].position,
					.. blockvbuffer[4][u]
					}
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
					Vertex{ 
					position: blockvbuffer[5][u].position,
					.. blockvbuffer[5][u]
					}
				);
			}
		}
	}
}
