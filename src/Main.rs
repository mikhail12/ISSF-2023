
use image::EncodableLayout;
use wgpu::{ColorTargetState, ComputePipelineDescriptor, BufferUsages, BindGroupEntry, ShaderStages, util::DeviceExt};
use winit::{
    event_loop::EventLoop,
    window::Window,
};

use std::{borrow::Cow, iter, num::{NonZeroU64, NonZeroU32}, array, any::TypeId};

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    
    let input = [1 as f32,3 as f32,7 as f32, 19 as f32, 18.2874];
    let mut res = input.clone();
    
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{ backends: wgpu::Backends::PRIMARY, dx12_shader_compiler: wgpu::Dx12Compiler::Fxc});
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
    

    let pipelineDescriptor = wgpu::ComputePipelineDescriptor {
        label: Some("doubling compute pipeline"),
        layout: None,
        module: &shader,
        entry_point: "computeSomething"
    };

    let pipeline = device.create_compute_pipeline( &pipelineDescriptor);


    let workBufferDescriptor = wgpu::BufferDescriptor {
        label: Some("Work Buffer"),
        size: bytemuck::bytes_of(&input).len() as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        mapped_at_creation: false
    };

    //let workBuffer = device.create_buffer(& workBufferDescriptor);
    //queue.write_buffer(&workBuffer,0, bytemuck::bytes_of(&input));

    let workBuffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Work Buffer"),
        contents: bytemuck::cast_slice(&input),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
    });

    let resultBufferDescriptor = wgpu::BufferDescriptor {
        label: Some("Result Buffer"),
        size: bytemuck::bytes_of(&input).len() as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false
    };

    let resultBuffer = device.create_buffer(&resultBufferDescriptor);

    let bindGroupLayout = pipeline.get_bind_group_layout(0);
    

    let bindGroupDescriptor = wgpu::BindGroupDescriptor {
        label: Some("Bindgroup for work buffer"),
        layout: &bindGroupLayout,
        entries: & [
            BindGroupEntry {binding: 0, resource: workBuffer.as_entire_binding()}
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
        pass.set_bind_group(0, &bindGroup, &[]);
        pass.dispatch_workgroups(input.len() as u32, 3, 3);
    }

    encoder.copy_buffer_to_buffer(&workBuffer, 0, &resultBuffer, 0, resultBuffer.size());
    
    queue.submit(iter::once(encoder.finish()));

    //let (sender, receiver) = futures::channel::oneshot::channel();
    
    
    //let buffer_future = slice.map_async(wgpu::MapMode::Read, |result| {
    //    let _ = sender.send(result);
    //});

    //device.poll(wgpu::Maintain::Wait);
    
    //let back = match receiver.await {
    //    Some(Ok(())) => {
    //        let data = slice.get_mapped_range();
    //        let result: [f32;3] = bytemuck::from_bytes::<[f32;3]>(data.as_bytes()).clone();
    //        print!("{:?}",result);
    //        
    //    },
    //    Err(e) => panic!("failed to run compute on gpu!")
    //}; 

    let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
    let slice = resultBuffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });
    // wait for the GPU to finish
    device.poll(wgpu::Maintain::Wait);

    match rx.receive().await {
        Some(Ok(())) => {
            let data = slice.get_mapped_range();
            res = data.chunks_exact(4).map(|b| f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<f32>>()
            .as_slice()
            .try_into()
            .unwrap();
            print!("{:?}",res);
            drop(data);
            resultBuffer.unmap();
        }
        _ => eprintln!("Something went wrong"),
    }
        
    

}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Tringle");
    env_logger::init();
    pollster::block_on( run(event_loop,window));
}
