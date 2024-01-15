use std::ops::{Add, Mul};

use anyhow::anyhow;
use nalgebra_glm as glm;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::gfx;

pub const WORLD_MAP: [[u8; 24]; 24] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

pub struct RaycastRenderer {
    canvas: Canvas<Window>,
    surface_size: glm::U32Vec2,
    target: Texture,
    texture_creator: TextureCreator<WindowContext>,

    pixels: Vec<glm::U8Vec3>,
}

impl RaycastRenderer {
    pub fn new(window: Window) -> anyhow::Result<Self> {
        let (width, height) = window.size();
        let canvas = window
            .into_canvas()
            .accelerated()
            .software()
            .target_texture()
            .build()?;

        let surface_size = glm::vec2(width, height);

        let texture_creator = canvas.texture_creator();
        let target =
            texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height)?;

        let size = (width * height) as usize;
        let pixels = Vec::with_capacity(size);

        let s = Self {
            canvas,
            surface_size,
            texture_creator,
            target,
            pixels,
        };
        Ok(s)
    }

    pub fn draw_map(
        &mut self,
        map: &[u8],
        cam_pos: glm::Vec2,
        face_dir: glm::Vec2,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub fn clear(&mut self, clear_color: impl Into<Option<gfx::Color>>) {
        let color = match clear_color.into() {
            Some(c) => c,
            None => gfx::Color::black(),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn raycast_screen(&mut self, player: &Player, cam: &Camera) {
        let w = self.surface_size.x;
        for x in 0..self.surface_size.x {
            let camx = (2 * x) as f32 / (w as f32) - 1.0;
            let ray_dir = cam.plane.mul(camx) + player.dir;

            let map_pos = glm::vec2(player.pos.x as i32, player.pos.y as i32);

            // length of ray from one x or y-side to next x or y-side
            let delta_dist = {
                let dx = if ray_dir.x == 0.0 {
                    std::f32::INFINITY
                } else {
                    (1.0 / ray_dir.x).abs()
                };

                let dy = if ray_dir.y == 0.0 {
                    std::f32::INFINITY
                } else {
                    (1.0 / ray_dir.y).abs()
                };
                glm::vec2(dx, dy)
            };

            let mut side_dist = glm::vec2(0., 0.);
            let mut perp_wall_dist = 0.;

            // what diretion to step in x or y-direction (either +1 or -1)
            let mut step = glm::vec2(0, 0);

            let mut hit = 0;  // was there a wall hit?
            let mut side = 0; // was a North-South or East-West wall hit
            

            if ray_dir.x < 0. {
                step.x = -1;
                side_dist.x = (player.pos.x -)
            }

            // if ray_dir.x == 0.0 || ray_dir.y == 0.0 {
            //     glm::vec2(std::f32::INFINITY, std::f32::INFINITY)
            // } else {
            //     ray_dir.abs().normalize()
            // };
        }
    }
}

struct Player {
    pos: glm::Vec2,
    dir: glm::Vec2,
}

struct Camera {
    plane: glm::Vec2,
}

pub fn run() -> anyhow::Result<()> {
    // Show logs from wgpu

    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        .position_centered()
        .resizable()
        .vulkan()
        .build()?;
    let (width, height) = window.size();

    let mut r = RaycastRenderer::new(window)?;

    let mut player = Player {
        pos: glm::vec2(22., 12.),
        dir: glm::vec2(-1., 0.),
    };
    let mut cam = Camera {
        plane: glm::vec2(0., 0.66),
    };

    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                e => {
                    dbg!(e);
                }
            }
        }

        r.raycast_screen(&player, &cam);
    }
    Ok(())
}
