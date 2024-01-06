mod geom;
mod gfx;
mod math;
mod render;
mod shader;

use anyhow::anyhow;
use raydium::render::QuadRenderer;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

fn main() -> anyhow::Result<()> {
    // Show logs from wgpu
    env_logger::init();

    let sdl_context = sdl2::init().map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow!(e))?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        .position_centered()
        .resizable()
        .vulkan()
        .build()?;
    let (width, height) = window.size();

    let mut r = smol::block_on(QuadRenderer::new(window))?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow!(e))?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    window_id,
                    win_event: WindowEvent::SizeChanged(width, height),
                    ..
                } if r.has_window(window_id) => {
                    r.resize(width as u32, height as u32);
                }
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

        // let frame = match surface.get_current_texture() {
        //     Ok(frame) => frame,
        //     Err(err) => {
        //         let reason = match err {
        //             SurfaceError::Timeout => "Timeout",
        //             SurfaceError::Outdated => "Outdated",
        //             SurfaceError::Lost => "Lost",
        //             SurfaceError::OutOfMemory => "OutOfMemory",
        //         };
        //         panic!("Failed to get current surface texture! Reason: {}", reason)
        //     }
        // };
        //
        // let output = frame
        //     .texture
        //     .create_view(&wgpu::TextureViewDescriptor::default());
        // let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //     label: Some("command_encoder"),
        // });
        //
        // {
        //     let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        //             view: &output,
        //             resolve_target: None,
        //             ops: wgpu::Operations {
        //                 load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
        //                 store: wgpu::StoreOp::Store,
        //             },
        //         })],
        //         depth_stencil_attachment: None,
        //         label: None,
        //         ..Default::default()
        //     });
        //     rpass.set_pipeline(&render_pipeline);
        //     rpass.set_bind_group(0, &bind_group, &[]);
        //     rpass.draw(0..3, 0..1);
        // }
        // queue.submit([encoder.finish()]);
        // frame.present();
    }

    Ok(())
}
