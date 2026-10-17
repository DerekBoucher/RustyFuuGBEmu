// Imported through a leading `::` because the `wgpu` child module shadows the
// crate name inside this module.
use ::wgpu::{Queue, RenderPass};

use crate::ppu;

mod shaders;
mod wgpu;

pub use self::wgpu::WgpuRenderer;

/// Technology-agnostic buffer of RGB pixel data that the emulator back end
/// sends to the front end whenever a frame is ready to be displayed.
pub type FrameData = [[ppu::Pixel; ppu::NATIVE_SCREEN_WIDTH]; ppu::NATIVE_SCREEN_HEIGHT];

/// Interface implemented by every rendering technology.
///
/// The application only depends on this trait, so rendering back ends can be
/// swapped, for example through a command line argument, without changing the
/// code that drives the emulator. Only [`FrameData`] crosses the boundary
/// between the emulator back end and the renderer, so the back end stays free
/// of rendering dependencies.
pub trait Renderer {
    /// Uploads the newest [`FrameData`] sent by the emulator back end.
    fn update_frame(&mut self, queue: &Queue, frame_data: FrameData);

    /// Draws the most recently uploaded frame.
    ///
    /// Called while the frame's render pass is recording, before the UI draws
    /// on top of it.
    fn render(&mut self, pass: &mut RenderPass<'static>);
}
