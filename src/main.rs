use glium::{glutin, Surface};

fn main(){
	println!("oof");
	
	//creating window
	let mut event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new().with_inner_size(glutin::dpi::LogicalSize::new(800.0, 800.0)).with_title("well this sucks");
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();
	
	#[derive(Copy, Clone)]
	struct Vertex {
		pos: [f32;3]
//		tex: [f32;2]
	}
	
	glium::implement_vertex!(Vertex, pos);
	
	let cube = vec![
		Vertex {pos: [0.0, 0.0, 0.0]},
		
		Vertex {pos: [0.0, 0.0, 0.0]}, // 1
		Vertex {pos: [0.0, 0.0, 1.0]}, // 2
		Vertex {pos: [1.0, 0.0, 1.0]}, // 3
		Vertex {pos: [1.0, 0.0, 0.0]}, // 4
		Vertex {pos: [0.0, 1.0, 0.0]}, // 5
		Vertex {pos: [0.0, 1.0, 1.0]}, // 6
		Vertex {pos: [1.0, 1.0, 1.0]}, // 7
		Vertex {pos: [1.0, 1.0, 0.0]}, // 8
	];
	
	let indices: Vec<u16> = vec![
		5, 
	];
	
	let v_buffer = glium::VertexBuffer::new(&display, &cube).unwrap();
	let i_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
	
	let vertex_shader_src = r#"
		#version 140
		
		in vec3 pos;
		
		uniform mat4 matrix;
		
		void main(){
			gl_Position = matrix * vec4(pos, 1.0);
		}
	"#;
	
	let fragment_shader_src = r#"
		#version 140
		
		out vec4 color
		
		void main(){
			color = vec4(1.0, 0.0, 0.0, 1.0);
		}
	"#;
	
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).expect("could not compile the vertex/fragment shader");
	
	event_loop.run(move| ev, _, control_flow| {
		let next_frame_time = std::time::Instant::now() +
		    std::time::Duration::from_nanos(16_666_667);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
		
		let matrix = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1f32]
		];
		
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);
		
		target.draw(&v_buffer, &i_buffer, &program, &glium::uniform!{ matrix: matrix }, &Default::default()).unwrap();
		
		target.finish().unwrap();
		
		match ev {
			glutin::event::Event::WindowEvent { event , .. } => match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
					return;	
				},
				_ => return,
			},
			_ => return,
		}
		
	})
}
