use learn_wgpu::app::App;
use winit::event_loop::EventLoop;

// ...
fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .filter(Some("learn_wgpu"), log::LevelFilter::Info)
        .init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
