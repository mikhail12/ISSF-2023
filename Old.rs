use gfx_hal::buffer;
use image::EncodableLayout;
use wgpu::{ColorTargetState, ComputePipelineDescriptor, BufferUsages, BindGroupEntry, ShaderStages};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use std::{borrow::Cow, num::{NonZeroU64, NonZeroU32}, array};

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    
    let input = [1 as f32,3 as f32,5 as f32];
    
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{ backends: wgpu::Backends::DX12, dx12_shader_compiler: wgpu::Dx12Compiler::Fxc});
    let surface = unsafe { instance.create_surface(&window)}.unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })
    .await
    .expect("Failed to find an appropriate adapter");

    let ( device , queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    ).await.unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let bindGroupLayoutEntryA = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer { 
            ty: wgpu::BufferBindingType::Storage { read_only: false }, 
            has_dynamic_offset: false, 
            min_binding_size: Some(NonZeroU64::new(bytemuck::bytes_of(&input).len() as u64).unwrap())},
        count: Some(NonZeroU32::new(input.len() as u32).unwrap())
    };

    let bindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("Work Bind Group Layout"),
        entries: &[bindGroupLayoutEntryA]
    };

    let bindGroupLayout = device.create_bind_group_layout(&bindGroupLayoutDescriptor);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Layout"),
        bind_group_layouts: &[&bindGroupLayout],
        push_constant_ranges: &[],
    });

    let pipelineDescriptor = wgpu::ComputePipelineDescriptor {
        label: Some("doubling compute pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "computeSomething"
    };

    let pipeline = device.create_compute_pipeline( &pipelineDescriptor);


    let workBufferDescriptor = wgpu::BufferDescriptor {
        label: Some("Work Buffer"),
        size: bytemuck::bytes_of(&input).len() as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: true
    };

    let workBuffer = device.create_buffer(& workBufferDescriptor);

    queue.write_buffer(&workBuffer,0, bytemuck::bytes_of(&input));

    let resultBufferDescriptor = wgpu::BufferDescriptor {
        label: Some("Result Buffer"),
        size: bytemuck::bytes_of(&input).len() as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: true
    };

    let resultBuffer = device.create_buffer(&resultBufferDescriptor);

    let bindGroupDescriptor = wgpu::BindGroupDescriptor {
        label: Some("Bindgroup for work buffer"),
        layout: &bindGroupLayout,
        entries: & [
            BindGroupEntry {binding: 0, resource: wgpu::BindingResource::Buffer( wgpu::BufferBinding {
                buffer: &workBuffer,
                offset: 0,
                size: Some(NonZeroU64::new(bytemuck::bytes_of(&input).len() as u64).unwrap())
            })}
        ]
    };

    let bindGroup = device.create_bind_group(&bindGroupDescriptor);

    let commandEncoderDescriptor = wgpu::CommandEncoderDescriptor {
        label: Some("doubling encoder")
    };

    let mut encoder = device.create_command_encoder(&commandEncoderDescriptor);
    
    let computePassDescriptor = wgpu::ComputePassDescriptor {
        label: Some("Doubling compute pass")
    };
    {
        let mut pass = encoder.begin_compute_pass(&computePassDescriptor);

        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bindGroup, &[0]);
        pass.dispatch_workgroups(3, 0, 0);
        pass.end_pipeline_statistics_query();
    }

    encoder.copy_buffer_to_buffer(&workBuffer, 0, &resultBuffer, 0, resultBuffer.size().clone());
    
    let commandBuffer = encoder.finish();
    queue.submit([commandBuffer]);

    let slice = resultBuffer.slice(..).get_mapped_range();
    let result: [f32;3] = bytemuck::from_bytes::<[f32;3]>(slice.as_bytes()).clone();
    print!("{:?}",result);
    
    resultBuffer.unmap();

    let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
    let slice = resultBuffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });
    // wait for the GPU to finish
    device.poll(wgpu::Maintain::Wait);

    //match rx.receive().await {
    //    Some(Ok(())) => {
    //        let data = slice.get_mapped_range();
    //        res = data.chunks_exact(4).map(|b| f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<f32>>()
    //        .as_slice()
    //        .try_into()
    //        .unwrap();
    //        print!("{:?}",res);
    //        drop(data);
    //        resultBuffer.unmap();
    //    }
    //    _ => eprintln!("Something went wrong"),
    //}
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Tringle");
    env_logger::init();
    pollster::block_on( run(event_loop,window));
}