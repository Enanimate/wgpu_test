use std::sync::Arc;
use library::state::State;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::GameLoop;

#[derive(Default)]
pub struct App<'a> {
    state: Option<State>,
    gameloop: Option<&'a GameLoop>
}

impl App<'_> {
    pub fn init(&mut self, gameloop: &GameLoop) {
        self.gameloop = Some(gameloop);
    } 
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        let gameloop = self.gameloop.as_mut().unwrap();
        if !gameloop.inputs(&event) {
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    state.render();
                    // Emits a new redraw requested event.
                    state.get_window().request_redraw();
                }
                WindowEvent::Resized(size) => {
                    // Reconfigures the size of the surface. We do not re-render
                    // here as this event is always followed up by redraw request.
                    state.resize(size);
                }
                _ => (),
            }
        }
    }
}