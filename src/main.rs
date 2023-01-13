mod cpu;
mod memory;

// extern crate glium;

// use glium::glutin;

fn main() {
    // // 1. The **winit::EventsLoop** for handling events.
    // let events_loop = glium::glutin::event_loop::EventLoop::new();
    // // 2. Parameters for building the Window.
    // let wb = glium::glutin::window::WindowBuilder::new()
    //     .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
    //     .with_title("Hello world");
    // // 3. Parameters for building the OpenGL context.
    // let cb = glium::glutin::ContextBuilder::new();
    // // 4. Build the Display with the given window and OpenGL context parameters and register the
    // //    window with the events_loop.
    // let _display = glium::Display::new(wb, cb, &events_loop).unwrap();

    // events_loop.run(move |ev, _, control_flow| {
    //     let next_frame_time =
    //         std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
    //     *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    //     match ev {
    //         glutin::event::Event::WindowEvent { event, .. } => match event {
    //             glutin::event::WindowEvent::CloseRequested => {
    //                 *control_flow = glutin::event_loop::ControlFlow::Exit;
    //                 return;
    //             }
    //             _ => return,
    //         },
    //         _ => (),
    //     }
    // });
}
