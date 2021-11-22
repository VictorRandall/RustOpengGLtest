use std::fs;
use std::fs::File;
use std::io::prelude::*;
use bincode::{serialize, deserialize, Infinite};
extern crate sdl2;
extern crate gl;
extern crate bincode;

struct Mesh{ // V.A.O
	vertbuffer: Vec<f32>,
	colorbuffer: Vec<f32>,
	uvbuffer: Vec<f32>,
	indexbuffer: Vec<usize>
}



fn main() -> std::io::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Rust Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
	
	let gl_context = window.gl_create_context().unwrap();
	
	let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
	
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                	println!("Exited");
                	break 'main Ok(());
                	},
                _ => {}
            }
        }
		
//		sus.iter_mut().for_each(|x|{
//			if x < &mut 1.0{
//				*x -= 0.0001;
//			}else {
//				*x += 0.001;
//			}
//		});
		
		unsafe{
			gl::ClearColor(0.4, 0.4, 0.8, 1.0);
		}
		
//		println!("red = {} green = {} blue = {} alpha = {}",sus[0], sus[1], sus[2], sus[3]);
		
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

//	let mut file = File::create("test.world")?;
//	let test = serialize("rust", Infinite).unwrap();
//		&Mesh{//               1        2         3       4      5        6        7        8     9
//			vertbuffer: vec![-0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.0f32],
//			indexbuffer: vec![1usize, 2usize, 3usize, 4usize, 5usize, 6usize, 7usize, 8usize, 9usize]
//		}, Infinite).unwrap();
//	file.write_all(&test)?;

//use std::fs;
//use std::fs::File;
//use std::io::prelude::*;
//
//#[macro_use]
//extern crate serde_derive;
//extern crate bincode;
//
//use bincode::{serialize, deserialize, Infinite};
//
//#[derive(Serialize, Deserialize, PartialEq, Debug)]
//struct Entity {
//    x: f32,
//    y: f32,
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Debug)]
//struct World(Vec<Entity>);
//
//fn main() -> std::io::Result<()> {
//    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
//
//    let mut encoded: Vec<u8> = serialize(&world, Infinite).unwrap();
//	
//	let mut file = File::create("test")?;
//	// Write a slice of bytes to the file
//	file.write_all(&encoded)?;
//	
//    // 8 bytes for the length of the vector, 4 bytes per float.
//    println!("{:?} {:?}", encoded.len(), 8 + 4 * 4);
//	
//	println!("{}",file.read_to_end(&mut encoded)?);
//    let decoded: World = deserialize(&encoded[..]).unwrap();
//
//    println!("{:?} {:?}", world, decoded);
//    
//    Ok(())
//}
