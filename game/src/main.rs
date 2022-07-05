mod application;
mod gfx;

use application::Application;

fn main() {
    Application::new("ubqpuzzle", (800, 600)).main_loop();
}
