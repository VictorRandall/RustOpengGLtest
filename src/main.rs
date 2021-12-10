#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

fn main(){
	let mut event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new();
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();
	
	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 2],
	}
	
	implement_vertex!(Vertex, position);
	
	let shape: Vec<Vertex; 3usize> = vec![
		Vertex { position: [-0.5, -0.5] }, 
		Vertex { position: [ 0.0,  0.5] }, 
		Vertex { position: [ 0.5, -0.25] }
	];
	
	let mut vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	
	event_loop.run(move |ev, _, control_flow| {
		
		let mut target = display.draw();
		target.clear_color(0.4,0.4,1.0,0.0);
		target.finish().unwrap();
	
		let next_frame_time = std::time::Instant::now() +
		    std::time::Duration::from_nanos(16_666_667);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
		match ev {
		    glutin::event::Event::WindowEvent { event, .. } => match event {
		        glutin::event::WindowEvent::CloseRequested => {
		            *control_flow = glutin::event_loop::ControlFlow::Exit;
		            return;
		        },
		        _ => return,
		    },
		    _ => (),
		}
	});
}
