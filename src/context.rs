use wgpu::{
    Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{game_error::GameError, timer::Timer};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub struct Context {
    pub timer: Timer,
    pub quit: bool,

    window: Window,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl Context {
    pub async fn new() -> Result<(Self, EventLoop<()>), GameError> {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

            WindowBuilder::new()
                .with_title("Durak")
                .with_inner_size(size)
                .build(&event_loop)?
        };

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .ok_or(GameError::new("Couldn't append canvas to document body."));
        }

        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.

        let surface = unsafe { instance.create_surface(&window)? };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(GameError::new("Cannot create adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        Limits::downlevel_webgl2_defaults()
                    } else {
                        Limits::default()
                    },
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);
        Ok((
            Context {
                timer: Timer::new(),
                quit: false,

                window,
                surface,
                device,
                queue,
                config,
                size,
            },
            event_loop,
        ))
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
}
