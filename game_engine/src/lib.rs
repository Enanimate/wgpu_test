use std::{mem::MaybeUninit, sync::Arc};
use library::state::State;
use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::PhysicalKey, window::{Window, WindowId}};
pub use winit::keyboard::KeyCode;

pub mod app;

#[derive(Clone)]
pub struct CaptureList {
    pub inputs: Box<[CaptureInput]>
}

#[derive(Clone, Copy)]
pub struct CaptureInput {
    pub key: KeyCode,
    pub function: fn()
}

pub trait ToCaptureList {
    fn to_list(self) -> CaptureList;
}

impl ToCaptureList for &[CaptureInput] {
    fn to_list(self) -> CaptureList {
        let mut boxed = Box::new_uninit_slice(self.len());
        for (index, val) in self.iter().enumerate() {
            boxed[index] = MaybeUninit::new(*val);
        }
    
        CaptureList {
            // Safe as all val inputs put into its slots are from an assumed initialized slice
            inputs: unsafe {boxed.assume_init()}
        }
    }
}

#[derive(Default)]
pub struct GameLoop {
    capture_events: Option<CaptureList>,
    state: Option<State>
}

impl GameLoop {
    pub fn new() -> Self {
        env_logger::init();

        Self {
            capture_events: None,
            state: Default::default()
        }
    }

    pub fn inputs(&self, event: &WindowEvent) -> bool {
        if self.capture_events.is_some() {
            match event {
                WindowEvent::KeyboardInput {event: KeyEvent{ physical_key, state, ..}, ..}
                    => {
                        for capture in self.capture_events.unwrap().inputs {
                            if *physical_key == capture.key || *state == ElementState::Released {

                            }
                        }
                    return true;
                }
                _ => false
            }
        } else {
            false
        }
    }
}

pub trait Events {
    fn capture_events(&mut self, function: fn() -> CaptureList) -> &mut Self;
    fn run(&mut self);
}

impl Events for GameLoop {
    fn capture_events(&mut self, function: fn() -> CaptureList) -> &mut Self {
        println!("flag2");
        self.capture_events = Some(function());
        return self
    }
    fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }
}

impl ApplicationHandler for GameLoop {
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
        if !self.inputs(&event) {
            let state = self.state.as_mut().unwrap();
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