use std::sync::mpsc;

use crate::graphics::{create_graphics, Graphics, Rc};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowAttributes,
};

enum State {
    Ready(Graphics),
    Init(Option<EventLoopProxy>),
}

pub struct App {
    state: State,
    sender: mpsc::Sender<Graphics>,
    receiver: mpsc::Receiver<Graphics>,
}

impl App {
    pub fn new(event_loop: &EventLoop) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            state: State::Init(Some(event_loop.create_proxy())),
            sender,
            receiver,
        }
    }

    fn draw(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.draw();
        }
    }

    fn resized(&mut self, size: PhysicalSize<u32>) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.resize(size);
        }
    }
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let mut win_attr = WindowAttributes::default();
        win_attr = win_attr.with_title("WebGPU example");
        let window = Rc::new(
            event_loop
                .create_window(win_attr)
                .expect("create window err."),
        );

        let proxy = event_loop.create_proxy();
        let sender = self.sender.clone();
        
        pollster::block_on(create_graphics(window, proxy, sender));
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::SurfaceResized(size) => self.resized(size),
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        if let Ok(graphics) = self.receiver.try_recv() {
            self.state = State::Ready(graphics);
        }
    }
}
