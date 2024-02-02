use std::ops::{Add, Mul};

use anyhow::anyhow;
use gfx::SDLTextureBuf;
use image::Pixel;
use nalgebra_glm as glm;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{PixelFormat, PixelFormatEnum},
    rect::{Point, Rect},
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

struct SDLContext {
    ctx: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

pub struct RaycastRenderer {
    sdl: SDLContext,
    target: SDLTextureBuf,
    texture_creator: TextureCreator<WindowContext>,
}

impl RaycastRenderer {
    pub fn new(ctx: Sdl, window: Window) -> anyhow::Result<Self> {
        let (width, height) = window.size();
        let canvas = window
            .into_canvas()
            .accelerated()
            .target_texture()
            .build()?;

        let texture_creator = canvas.texture_creator();

        let event_pump = ctx.event_pump().map_err(|e| anyhow!(e))?;

        let sdl = SDLContext {
            ctx,
            canvas,
            event_pump,
        };

        let target = SDLTextureBuf::new(&texture_creator, width, height)?;

        let s = Self {
            sdl,
            texture_creator,
            target,
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

    pub fn clear(&mut self, clear_color: impl Into<Option<gfx::Color>>) -> anyhow::Result<()> {
        let color = match clear_color.into() {
            Some(c) => c,
            None => gfx::Color::black(),
        };
        self.sdl.canvas.set_draw_color(color);
        self.sdl.canvas.clear();
        self.target.clear_black();
        Ok(())
    }

    fn raycast_screen(&mut self, player: &Player, cam: &Camera) -> anyhow::Result<()> {
        let w = self.target.width();
        let h = self.target.height();
        for x in 0..w {
            let camx = (2 * x) as f32 / (w as f32) - 1.0;
            let ray_dir = player.dir + cam.plane * camx; // cam.plane.mul(camx) + player.dir;

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

            {
                let mapx = map_pos.x as f32;
                let mapy = map_pos.y as f32;
                let playerx = player.pos.x;
                let playery = player.pos.y;

                if ray_dir.x < 0. {
                    step.x = -1;
                    side_dist.x = (playerx - mapx) * delta_dist.x;
                } else {
                    step.x = 1;
                    side_dist.x = (mapx + 1.0 - playerx) * delta_dist.x;
                }

                if ray_dir.y < 0.0 {
                    step.y = -1;
                    side_dist.y = (playery - mapy) * delta_dist.y;
                } else {
                    step.y = 1;
                    side_dist.y = (mapy + 1.0 - playery) * delta_dist.y;
                }
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
            let line_height = (h as f32 / perp_wall_dist) as i32;

            // calc loweest and highest pixel to fill in current stripe
            let h = h as i32;
            let mut draw_start = -line_height / 2 + h / 2;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end = line_height / 2 + h / 2;
            if draw_end >= h {
                draw_end = h - 1;
            }

            // choose wall color
            let color = {
                let x = map_pos.x as usize;
                let y = map_pos.y as usize;
                let mut col: [u8; 3] = match WORLD_MAP[x][y] {
                    1 => [255, 0, 0],
                    2 => [0, 255, 0],
                    3 => [0, 0, 255],
                    4 => [218, 112, 214], // Orchid
                    _ => [255, 255, 255],
                };

                // give x and y side different brightness
                if side == 1 {
                    for c in col.iter_mut() {
                        *c = *c / 2;
                    }
                    // let [r, g, b] = &mut color.to_rgb();
                    // color. = color.0[0] / 2;
                    // color.g = color.g / 2;
                    // color.b = color.b / 2;
                }

                image::Rgb(col)
            };

            let _ = self.draw_vert_line(x as i32, draw_start, draw_end, color)?;
        }
        Ok(())
    }

    // Fast vertical line from (x, y1) to (x, y2) with rgb color.
    fn draw_vert_line(
        &mut self,
        x: i32,
        mut y1: i32,
        mut y2: i32,
        color: image::Rgb<u8>,
    ) -> anyhow::Result<bool> {
        let w = self.target.width() as i32;
        let h = self.target.height() as i32;

        // swap y1 and y2
        if y2 < y1 {
            y1 += y2;
            y2 = y1 - y2;
            y1 -= y2;
        }
        if y2 < 0 || y1 >= h as i32 || x < 0 || x >= w as i32 {
            // no single point of the line is on screen.
            return Ok(false);
        }

        // clip
        if y1 < 0 {
            y1 = 0;
        }

        // clip
        if y2 >= w as i32 {
            y2 = h as i32 - 1
        }

        // self.sdl.canvas.set_draw_color(*color);
        // self.sdl
        //     .canvas
        //     .draw_line(Point::new(x, y1), Point::new(x, y2));
        // for y in y1..=y2 {
        //     self.sdl
        //         .canvas
        //         .draw_point(Point::new(x, y))
        //         .map_err(|e| anyhow!(e))?;
        // }
        //

        let x = x as u32;
        for y in y1..=y2 {
            let y = y as u32;
            self.target.put(x, y, color);
        }

        Ok(true)
    }

    fn present(&mut self) -> anyhow::Result<()> {
        self.target.flush()?;
        self.target.draw(&mut self.sdl.canvas)?;
        // self.sdl
        //     .canvas
        //     .copy(
        //         &self.target,
        //         None,
        //         None
        //     )
        //     .map_err(|e| anyhow!(e))?;
        self.sdl.canvas.present();
        Ok(())
    }
}

struct Player {
    pos: glm::Vec2,
    dir: glm::Vec2,
    speed: f32,
}

struct Camera {
    plane: glm::Vec2,
    speed: f32,
}

pub fn run() -> anyhow::Result<()> {
    // Show logs from wgpu

    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        // .position_centered()
        // .resizable()
        .vulkan()
        .build()?;
    let (width, height) = window.size();

    let mut r = RaycastRenderer::new(sdl_context, window)?;

    let mut player = Player {
        pos: glm::vec2(22., 12.),
        dir: glm::vec2(-1., 0.),
        speed: 25.,
    };
    let mut cam = Camera {
        plane: glm::vec2(0., 0.66),
        speed: 35.,
    };

    let mut last_dt = std::time::Instant::now();

    'running: loop {
        let now = std::time::Instant::now();
        let dt = (now - last_dt).as_secs_f32();
        last_dt = now;

        for event in r.sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let next_x = (player.pos.x + player.dir.x * (player.speed * dt)) as usize;
                    let next_y = (player.pos.y + player.dir.y * (player.speed * dt)) as usize;
                    if WORLD_MAP[next_x][player.pos.y as usize] == 0 {
                        player.pos.x += player.dir.x * (player.speed * dt);
                    }
                    if WORLD_MAP[player.pos.x as usize][next_y] == 0 {
                        player.pos.y += player.dir.y * (player.speed * dt);
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let next_x = (player.pos.x + player.dir.x * (player.speed * dt)) as usize;
                    let next_y = (player.pos.y + player.dir.y * (player.speed * dt)) as usize;
                    if WORLD_MAP[next_x][player.pos.y as usize] == 0 {
                        player.pos.x -= player.dir.x * (player.speed * dt);
                    }
                    if WORLD_MAP[player.pos.x as usize][next_y] == 0 {
                        player.pos.y -= player.dir.y * (player.speed * dt);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let old_dx = player.dir.x;
                    let rot_speed = cam.speed * dt;

                    player.dir.x =
                        player.dir.x * (-rot_speed).cos() - player.dir.y * (-rot_speed).sin();
                    player.dir.y = old_dx * (-rot_speed).sin() + player.dir.y * (-rot_speed).cos();

                    let old_px = cam.plane.x;
                    cam.plane.x =
                        cam.plane.x * (-rot_speed).cos() - cam.plane.y * (-rot_speed).sin();
                    cam.plane.y = old_px * (-rot_speed).sin() + cam.plane.y * (-rot_speed).cos();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let old_dx = player.dir.x;
                    let rot_speed = cam.speed * dt;

                    player.dir.x =
                        player.dir.x * (rot_speed).cos() - player.dir.y * (rot_speed).sin();
                    player.dir.y = old_dx * (rot_speed).sin() + player.dir.y * (rot_speed).cos();

                    let old_px = cam.plane.x;
                    cam.plane.x = cam.plane.x * (rot_speed).cos() - cam.plane.y * (rot_speed).sin();
                    cam.plane.y = old_px * (rot_speed).sin() + cam.plane.y * (rot_speed).cos();
                }
                e => {
                    dbg!(e);
                }
            }
        }

        r.clear(None)?;
        // r.raycast_screen(&player, &cam)?;
        r.present()?;
    }
    Ok(())
}
