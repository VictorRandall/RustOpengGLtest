use std::collections::HashMap;
use std::file;

use crate::mesh::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum VoxelType{
	Air,
	Solid,
//	grass,
//	dirt,
//	stone,
//	cobblestone,
//	water(usize)
}

#[derive(Debug, Clone)]
pub struct VoxelChunk{
	blocks: HashMap<[i8;3], VoxelType>,
	position: [i16;3],
	chunk_size: usize
}

impl VoxelChunk{
	pub fn new(pos: [i16;3]) -> Self{
		Self {
			blocks: HashMap::<[i8;3], VoxelType>::new(),
			position: pos,
			chunk_size: 2usize
		}
	}

	fn generate_data(&mut self,pos: [i8;3]){//gerar os blocos
		self.blocks.insert(pos, VoxelType::Solid);
	}
	
	fn get_block(&self,pos: [i8;3]) -> VoxelType{
//		let voxel = self.blocks.get(&pos);
//		if pos[0] < 0i8 || pos[1] < 0i8 || pos[2] < 0i8{
//			return VoxelType::Air;
//		}
		println!("{:?} {:?}", self.blocks.get(&[pos[0], pos[1], pos[2]]), [pos[0], pos[1], pos[2]]);
		if pos[0] > self.chunk_size as i8 && pos[0] < 0i8 && pos[1] > self.chunk_size as i8 && pos[1] < 0i8 && pos[2] > self.chunk_size as i8 && pos[2] < 0i8 {
//			*self.blocks.get(&[pos[0] as usize, pos[1] as usize, pos[2] as usize]).unwrap()
			return VoxelType::Air;
		}
		match self.blocks.get(&[pos[0], pos[1], pos[2]]) {
			Some(v) => return *v,
			None => return VoxelType::Air
		}
//		} else { VoxelType::Air }
	}
	
	pub fn generate_mesh(&mut self, display: &glium::Display) -> Mesh{//gerar o modelo 3d dos blocos
		let mut vertexbuffer: Vec<Vertex> = vec![];
//			Vertex{ position: [0.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },//botton 0
//			Vertex{ position: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },//		
//			Vertex{ position: [0.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },//
//			Vertex{ position: [1.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },//
//				
//			Vertex{ position: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },//top 4
//			Vertex{ position: [1.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },//  
//			Vertex{ position: [0.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },// 
//			Vertex{ position: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },//
//			
//			Vertex{ position: [0.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },//left 8
//			Vertex{ position: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] },
//			Vertex{ position: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] },
//			Vertex{ position: [1.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] },
//			
//			Vertex{ position: [0.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },//right 12
//			Vertex{ position: [1.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.0] },
//			Vertex{ position: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.5] },
//			Vertex{ position: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coords: [0.3, 0.5] },
//			
//			Vertex{ position: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },//front 16
//			Vertex{ position: [1.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.0] },
//			Vertex{ position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.5] },
//			Vertex{ position: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.5] },
//			
//			Vertex{ position: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },//back 20
//			Vertex{ position: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.0] },
//			Vertex{ position: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.5] },
//			Vertex{ position: [0.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coords: [0.3, 0.5] },
//		];
		
		let mut indexbuffer: Vec<u16> = vec![];
//			2u16, 1, 0,//top
//			2, 3, 1,
//		
//			4, 5, 6,   //botton
//			5, 7, 6,
//			
//			8, 9, 10,  //left
//			9, 11, 10,
//			
//			14, 13, 12,//right
//			14, 15, 13,
//			
//			16, 17, 18,//front
//			17, 19, 18,
//			
//			22, 21, 20,//back
//			22, 23, 21, 
//		];
		
		
		for x in 0..self.chunk_size{
			for y in 0..self.chunk_size{
				for z in 0..self.chunk_size{
					self.generate_data([x as i8,y as i8,z as i8]);
					
//					println!("sus");
					self.create_block([3usize,2], [x as i8,y as i8,z as i8], &mut vertexbuffer, &mut indexbuffer);
//					self.create_block([3usize,2], [0i8,0i8,0i8], &mut vertexbuffer, &mut indexbuffer);
				}
			}
		}
		
//		println!("{:?} {:?}", vertexbuffer, indexbuffer);
		println!("{:?}", self.blocks);
		
		return Mesh::new(
			glium::VertexBuffer::new(display, &vertexbuffer.as_slice()).unwrap(),
			glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indexbuffer.as_slice()).unwrap(),
			[
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 2.0, 1.0f32]]);
	}
	fn create_block(&self,blocks: [usize;2], pos: [i8;3], vertexbuffer: &mut Vec<Vertex>, indexbuffer: &mut Vec<u16>){
		let blockvbuffer = [
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
		
//		if self.get_block([pos[0],pos[1] + 1i8,pos[2]]) == VoxelType::Air{ return; }
		
		if self.get_block([pos[0],pos[1] + 1i8,pos[2]]) == VoxelType::Air{ //top
			println!("top");
			for u in 0..blockibuffer[0].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[0][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[0].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[0][u]);
			}
		}
		if self.get_block([pos[0],pos[1] - 1i8,pos[2]]) == VoxelType::Air{ //botton
			println!("botton");
			for u in 0..blockibuffer[1].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[1][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[1].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[1][u]);
			}
		}
		if self.get_block([pos[0] - 1i8,pos[1],pos[2]]) == VoxelType::Air{ //left
			println!("left");
			for u in 0..blockibuffer[2].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[2][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[2].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[2][u]);
			}
		}
		if self.get_block([pos[0] + 1i8,pos[1],pos[2]]) == VoxelType::Air{ //right
			println!("right");
			for u in 0..blockibuffer[3].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[3][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[3].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[3][u]);
			}
		}
		if self.get_block([pos[0],pos[1],pos[2] + 1i8]) == VoxelType::Air{ //front
			println!("front");
			for u in 0..blockibuffer[4].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[4][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[4].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[4][u]);
			}
		}
		if self.get_block([pos[0],pos[1],pos[2] - 1i8]) == VoxelType::Air{ //back
			println!("back");
			for u in 0..blockibuffer[5].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[5][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[5].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[5][u]);
			}
		}
//		println!("{:?} {:?}", vertexbuffer, indexbuffer);
	}
}
