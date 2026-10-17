use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

/// Rendering backend selected with the `--renderer` command line argument.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum Backend {
    /// Let wgpu pick the best available backend.
    #[default]
    Auto,
    /// Force the Vulkan backend.
    Vulkan,
    /// Force the OpenGL backend.
    #[value(name = "opengl")]
    OpenGl,
}

impl Backend {
    /// The wgpu backend mask this selection restricts wgpu to.
    fn backends(self) -> wgpu::Backends {
        match self {
            Self::Auto => wgpu::Backends::PRIMARY,
            Self::Vulkan => wgpu::Backends::VULKAN,
            Self::OpenGl => wgpu::Backends::GL,
        }
    }
}

/// wgpu objects shared by the UI and the render loop.
///
/// Owns everything needed to present frames: the configured surface, the
/// device/queue work is submitted to, and the swapchain configuration kept up
/// to date by [`GpuContext::resize`] and [`GpuContext::reconfigure`].
pub struct GpuContext {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    pub fn new(window: Arc<Window>, backend: Backend) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: backend.backends(),
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.clone())
            .map_err(|err| format!("failed to create surface: {err}"))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .map_err(|err| format!("failed to find a compatible adapter: {err}"))?;

        let adapter_info = adapter.get_info();
        log::info!(
            "wgpu adapter: {} ({:?}, {:?} backend)",
            adapter_info.name,
            adapter_info.device_type,
            adapter_info.backend
        );

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                label: Some("rusty_fuugbemu"),
                ..Default::default()
            }))
            .map_err(|err| format!("failed to create device: {err}"))?;

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or_else(|| "surface is not supported by the selected adapter".to_string())?;

        let caps = surface.get_capabilities(&adapter);

        // Both the Game Boy frame and egui's colours are already in gamma
        // space, so a non-sRGB swapchain format is preferred: values are
        // written as-is instead of being run through the hardware
        // linear-to-sRGB conversion, matching how OpenGL presented before.
        if let Some(format) = caps
            .formats
            .iter()
            .copied()
            .find(|format| {
                matches!(
                    format,
                    wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Bgra8Unorm
                )
            })
        {
            config.format = format;
        }

        // `get_default_config` picked `Mailbox` for this surface, which the
        // Vulkan Wayland driver reports as a suboptimal present on every
        // frame. `Fifo` presents without tearing and matches the emulator's
        // 60 FPS pacing.
        config.present_mode = wgpu::PresentMode::Fifo;

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
        })
    }

    /// Resizes the swapchain to match the window's new physical size.
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.reconfigure();
    }

    /// Rebuilds the swapchain, e.g. after a resize or when the surface was
    /// reported as lost or outdated.
    pub fn reconfigure(&self) {
        self.surface.configure(&self.device, &self.config);
    }
}
