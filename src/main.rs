mod geom;
mod gfx;
mod math;
mod raycast;
mod render;
mod shader;

use anyhow::anyhow;
use raycast::run;
use raydium::render::QuadRenderer;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

fn main() -> anyhow::Result<()> {
    run()
}
