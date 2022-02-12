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
	blocks: HashMap<[usize;3], VoxelType>,
	position: [i16;3],
	chunk_size: usize
}

impl VoxelChunk{
	pub fn new(pos: [i16;3]) -> Self{
		Self {
			blocks: HashMap::<[usize;3], VoxelType>::new(),
			position: pos,
			chunk_size: 4usize
		}
	}

	fn generate_data(&mut self,pos: [usize;3]){//gerar os blocos
		self.blocks.insert(pos, VoxelType::Solid);
	}
	
	fn get_block(&self,pos: [usize;3]) -> VoxelType{ // TODO: use self.chunk_size instead of a parameter
//		if pos[0] < self.chunk_size && pos[0] > 0usize && pos[1] < self.chunk_size && pos[1] > 0usize && pos[2] < self.chunk_size && pos[2] > 0usize {
//			*self.blocks.get(&pos).unwrap()
//		} else { VoxelType::Air }
		let voxel = self.blocks.get(&pos);
		
		match voxel {
			Some(v) => return *v,
			None => return VoxelType::Air
		}
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
					self.generate_data([x,y,z]);
					
					println!("sus");
					self.create_block([3usize,2], [x,y,z], &mut vertexbuffer, &mut indexbuffer);
				}
			}
		}
		
		println!("{:?} {:?}", vertexbuffer, indexbuffer);
		
		return Mesh::new(
			glium::VertexBuffer::new(display, &vertexbuffer.as_slice()).unwrap(),
			glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indexbuffer.as_slice()).unwrap(),
			[
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 2.0, 1.0f32]]);
	}
	fn create_block(&self,blocks: [usize;2], pos: [usize;3], vertexbuffer: &mut Vec<Vertex>, indexbuffer: &mut Vec<u16>){
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
				2, 3, 2,
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
		
		
		
		if self.get_block([pos[0],pos[1] + 1usize,pos[2]]) == VoxelType::Air{ //top
			println!("sus");
			for u in 0..blockibuffer[0].len(){
				indexbuffer.insert(indexbuffer.len(), blockibuffer[0][u] + vertexbuffer.len() as u16);
			}
			for u in 0..blockvbuffer[0].len(){
				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[0][u]);
			}
		}
//		if self.get_block([pos[0],pos[1] - 1usize,pos[2]]) == VoxelType::Air{ //botton
//			for u in 0..blockvbuffer[1].len(){
//				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[1][u]);
//			}
//		}
//		if self.get_block([pos[0] - 1usize,pos[1],pos[2]]) == VoxelType::Air{ //left
//			for u in 0..blockvbuffer[2].len(){
//				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[2][u]);
//			}
//		}
//		if self.get_block([pos[0] + 1usize,pos[1],pos[2]]) == VoxelType::Air{ //right
//			for u in 0..blockvbuffer[3].len(){
//				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[3][u]);
//			}
//		}
//		if self.get_block([pos[0],pos[1],pos[2] + 1usize]) == VoxelType::Air{ //front
//			for u in 0..blockvbuffer[4].len(){
//				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[4][u]);
//			}
//		}
//		if self.get_block([pos[0],pos[1],pos[2] - 1usize]) == VoxelType::Air{ //back
//			for u in 0..blockvbuffer[5].len(){
//				vertexbuffer.insert(vertexbuffer.len(), blockvbuffer[5][u]);
//			}
//		}
	}
}
