use crate::state::State;
use std::sync::Arc;
use wgpu::SurfaceError;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl App {
    fn handle_keyboard(&mut self, key_event: KeyEvent, active_event_loop: &ActiveEventLoop) {
        match key_event {
            KeyEvent {
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
            } => active_event_loop.exit(),
            _ => {}
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_visible(false))
                .unwrap(),
        );
        self.state = Some(pollster::block_on(State::new(window.clone())).unwrap());
        window.set_visible(true);
        
        log::info!("App resumed");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => {
                
                match state.render() {
                    Ok(_) => {}
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        state.configure_surface();
                    }
                    Err(e) => {
                        log::error!("Render Error: {:?}", e);
                    }
                }
            },
            WindowEvent::KeyboardInput { event, .. } => self.handle_keyboard(event, event_loop),
            _ => (),
        }
    }
}
