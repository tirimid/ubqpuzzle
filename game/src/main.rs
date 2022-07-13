mod application;
mod gfx;
mod types;

use application::Application;

fn main() {
    Application::new("ubqpuzzle", (1600, 900)).main_loop();
}
