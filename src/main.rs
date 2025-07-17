use winit::event_loop::EventLoop;

use crate::app::App;

mod state;
mod app;
mod application_handler;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let event_loop = EventLoop::with_user_event().build()?;

    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}

