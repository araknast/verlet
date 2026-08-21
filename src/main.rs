mod renderer;
mod solver;

use crate::renderer::Renderer;
use crate::solver::Solver;
use sfml::SfResult;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Event, Style};

fn main() -> SfResult<()> {
    const WIN_W: u32 = 800;
    const WIN_H: u32 = 600;
    const WIN_TITLE: &str = "New Window";
    const BALL_SIZE: f32 = 10.;

    let fps_limit = 60;
    let aa_level = 5;

    let context_settings = ContextSettings {
        antialiasing_level: aa_level,
        ..Default::default()
    };

    let mut window = RenderWindow::new([WIN_W, WIN_H], WIN_TITLE, Style::CLOSE, &context_settings)?;

    window.set_framerate_limit(fps_limit);

    let constraint_center = Vector2f::new(WIN_W as f32 * 0.5, WIN_H as f32 * 0.5);
    let mut solver = Solver::new(8, 1000., constraint_center, 250.0, fps_limit);

    solver.add_object(BALL_SIZE, None);

    let mut renderer;

    loop {
        if let Some(event) = window.poll_event() {
            match event {
                Event::MouseButtonPressed { x, y, .. } => {
                    solver.add_object(BALL_SIZE, Some(Vector2f::new(x as f32, y as f32)));
                }
                _ => (),
            }
        }

        solver.update();
        window.clear(Color::WHITE);
        renderer = Renderer::new(&mut *window);
        renderer.render(&mut solver);
        window.display();
    }
}
