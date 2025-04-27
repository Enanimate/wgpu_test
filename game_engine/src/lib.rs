use std::{mem::MaybeUninit, slice::SliceIndex};

use app::App;
use winit::{event::{ElementState, KeyEvent, WindowEvent}, event_loop::{ControlFlow, EventLoop}, keyboard::PhysicalKey};
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

#[derive(Clone)]
pub struct GameLoop {
    capture_events: Option<CaptureList>
}

impl GameLoop {
    pub fn new() -> Self {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
    
        let gameloop = Self {
            capture_events: None
        };

        let mut app = App::default();
        App::init(&mut app, &gameloop);
        
        event_loop.run_app(&mut app).unwrap();

        return gameloop;
    }

    pub fn inputs(&mut self, event: &WindowEvent) -> bool {
        if self.capture_events.is_some() {
            match event {
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state,
                            physical_key: PhysicalKey::Code(KeyCode::KeyE),
                            ..
                        },
                    ..
                } => {
                    if *state == ElementState::Released {
                        println!("TEST");
                    }
                    true
                }
                _ => false
            }
        } else {
            false
        }
    }
}

pub trait Events {
    fn capture_events(&mut self, function: fn() -> CaptureList);
}

impl Events for GameLoop {
    fn capture_events(&mut self, function: fn() -> CaptureList) {
        self.capture_events = Some(function());
    }
}