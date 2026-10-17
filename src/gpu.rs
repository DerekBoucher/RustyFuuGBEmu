use std::sync::Arc;

use winit::window::Window;

/// wgpu objects shared by the render loop and the UI.
///
/// Created once at startup. Rendering still goes through glium; the renderer
/// and the egui painter will start drawing through these objects when the
/// render loop is migrated to wgpu.
// TODO(wgpu-migration): the fields are read once the render loop uses wgpu;
// drop this allow in the next step.
#[allow(dead_code)]
pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    pub fn new(window: Arc<Window>) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
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
        let config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or_else(|| "surface is not supported by the selected adapter".to_string())?;

        // NOTE: `surface.configure(&device, &config)` is deliberately not
        // called yet. Creating the Vulkan swapchain puts the window's Wayland
        // surface into explicit-sync mode, which breaks GL presentation on the
        // same surface (`wp_linux_drm_syncobj_surface_v1` protocol error). It
        // moves here as soon as glium is removed and wgpu owns presentation.

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
        })
    }
}
