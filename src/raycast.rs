use std::ops::{Add, Mul};

use anyhow::anyhow;
use nalgebra_glm as glm;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{PixelFormat, PixelFormatEnum},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl,
};

use crate::gfx;

const MAP_W: usize = 24;
const MAP_H: usize = 24;

pub const WORLD_MAP: [[u8; MAP_W]; MAP_H] = [
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

struct SdlContext {
    ctx: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

pub struct RaycastRenderer {
    sdl: SdlContext,
    surface_size: glm::U32Vec2,
    target: Texture,
    texture_creator: TextureCreator<WindowContext>,

    pixels: Vec<u32>,
}

impl RaycastRenderer {
    pub fn new(ctx: Sdl, window: Window) -> anyhow::Result<Self> {
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
        let event_pump = ctx.event_pump().map_err(|e| anyhow!(e))?;

        let sdl = SdlContext {
            ctx,
            canvas,
            event_pump,
        };

        let s = Self {
            sdl,
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
        self.sdl.canvas.set_draw_color(color);
        self.sdl.canvas.clear();
    }

    fn raycast_screen(&mut self, player: &Player, cam: &Camera) {
        let w = self.surface_size.x;
        for x in 0..self.surface_size.x {
            let camx = (2 * x) as f32 / (w as f32) - 1.0;
            let ray_dir = cam.plane.mul(camx) + player.dir;

            let mut map_pos = glm::vec2(player.pos.x as i32, player.pos.y as i32);

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

            let mut hit = 0; // was there a wall hit?
            let mut side = 0; // was a North-South or East-West wall hit

            if ray_dir.x < 0. {
                step.x = -1;
                side_dist.x = (player.pos.x - map_pos.x as f32) * delta_dist.x;
            } else {
                step.x = 1;
                side_dist.x = (map_pos.x as f32 + 1.0 - player.pos.x) * delta_dist.x;
            }

            if ray_dir.y < 0.0 {
                step.y = -1;
                side_dist.y = (player.pos.x - map_pos.x as f32) * delta_dist.y;
            } else {
                step.y = 1;
                side_dist.y = (map_pos.y as f32 + 1.0 - player.pos.y) * delta_dist.y;
            }

            // DDA
            while hit == 0 {
                // jump to next map square, either in x or y-direction
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x;
                    side = 0;
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y;

                    side = 1;
                }

                // Check if ray has hit a wall
                if WORLD_MAP[map_pos.x as usize][map_pos.y as usize] > 0 {
                    hit = 1;
                }
            }

            if side == 0 {
                perp_wall_dist = side_dist.x - delta_dist.x;
            } else {
                perp_wall_dist = side_dist.y - delta_dist.y;
            }

            // calc height of line to draw on screen
            let line_height = (MAP_H as f32 / perp_wall_dist) as i32;

            // calc loweest and highest pixel to fill in current stripe
            const MH: i32 = MAP_H as i32;
            let mut draw_start = -line_height / 2 + MH / 2;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end = line_height / 2 + MH / 2;
            if draw_end >= MH {
                draw_end = MH - 1;
            }

            // choose wall color
            let mut color = match WORLD_MAP[map_pos.x as usize][map_pos.y as usize] {
                1 => sdl2::pixels::Color::RED,
                2 => sdl2::pixels::Color::GREEN,
                3 => sdl2::pixels::Color::BLUE,
                4 => sdl2::pixels::Color::WHITE,
                _ => sdl2::pixels::Color::YELLOW,
            };

            // give x and y side different brightness
            if side == 1 {
                color.r = color.r / 2;
                color.g = color.g / 2;
                color.b = color.b / 2;
            }

            self.draw_vert_line(x as i32, draw_start, draw_end, &color);
        }
    }

    // Fast vertical line from (x, y1) to (x, y2) with rgb color.
    fn draw_vert_line(
        &mut self,
        x: i32,
        mut y1: i32,
        mut y2: i32,
        color: &sdl2::pixels::Color,
    ) -> anyhow::Result<bool> {
        // swap y1 and y2
        if y2 < y1 {
            y1 += y2;
            y2 = y1 - y2;
            y1 -= y2;
        }
        if y2 < 0 || y1 >= MAP_H as i32 || x < 0 || x >= MAP_W as i32 {
            // no single point of the line is on screen.
            return Ok(false);
        }

        // clip
        if y1 < 0 {
            y1 = 0;
        }

        // clip
        if y2 >= MAP_W as i32 {
            y2 = MAP_H as i32 - 1
        }

        let mut surface = self
            .sdl
            .canvas
            .window()
            .surface(&self.sdl.event_pump)
            .map_err(|e| anyhow!(e))?;
        let color = {
            let format = surface.pixel_format();
            color.to_u32(&format)
        };

        let pitch = surface.pitch() as i32;

        surface.with_lock_mut(|pixels| {
            let pixels: &mut [u32] = unsafe { std::mem::transmute(pixels) };

            let mut bufp = pixels
                .iter_mut()
                .skip((y1 * pitch / 4) as usize)
                .skip(x as usize);
            let add = (pitch / 4) as usize;

            for y in y1..=y2 {
                let mut b = bufp.next().unwrap();
                *b = color;
                bufp.skip(add);
            }
        });

        Ok(true)
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

    let mut r = RaycastRenderer::new(sdl_context, window)?;

    let mut player = Player {
        pos: glm::vec2(22., 12.),
        dir: glm::vec2(-1., 0.),
    };
    let mut cam = Camera {
        plane: glm::vec2(0., 0.66),
    };

    'running: loop {
        for event in r.sdl.event_pump.poll_iter() {
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
