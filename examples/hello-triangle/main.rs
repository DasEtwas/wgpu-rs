use std::num::NonZeroU32;
use winit::window::Window;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::DX12);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, _queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty() | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY,
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // crashes
    let crash_parameter_binding_a = 0;
    let crash_parameter_binding_b = 2;
    let crash_parameter_count_a = 10;
    let crash_parameter_count_b = 2;

    // works
    /*
    let crash_parameter_binding_a = 0;
    let crash_parameter_binding_b = 2;
    let crash_parameter_count_a = 1;
    let crash_parameter_count_b = 2;
    */

    let vertex = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStage::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: Some(NonZeroU32::new(crash_parameter_count_b).unwrap()),
    };

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: crash_parameter_binding_a,
                count: Some(NonZeroU32::new(crash_parameter_count_a).unwrap()),
                ..vertex.clone()
            },
            wgpu::BindGroupLayoutEntry {
                binding: crash_parameter_binding_b,
                ..vertex
            },
        ],
    });

    // crashes with some bind group layouts
    let test_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(format!("test pipeline layout").as_str()),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    wgpu_subscriber::initialize_default_subscriber(None);
    // Temporarily avoid srgb formats for the swapchain on the web
    pollster::block_on(run(event_loop, window));
}
