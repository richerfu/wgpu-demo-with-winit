mod app;
mod graphics;

use crate::app::App;
use openharmony_ability::OpenHarmonyApp;
use openharmony_ability_derive::ability;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::ohos::EventLoopBuilderExtOpenHarmony;

fn run_app(event_loop: EventLoop, app: App) {
    // Allows the setting of the log level through RUST_LOG env var.
    // It also allows wgpu logs to be seen.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    // Runs the app on the current thread.
    let _ = event_loop.run_app(app);
}

#[ability]
fn open(app: OpenHarmonyApp) {
    // <T> (T -> AppEvent) extends regular platform specific events (resize, mouse, etc.).
    // This allows our app to inject custom events and handle them alongside regular ones.
    // let event_loop = EventLoop::<()>::new().unwrap();
    let event_loop = EventLoop::builder()
        .with_openharmony_app(app)
        .build()
        .unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);

    let app = App::new(&event_loop);
    run_app(event_loop, app);
}
