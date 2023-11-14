


use futures::FutureExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use wgpu::util::DeviceExt;

use array_macro::array;

use std::{borrow::Cow, iter, num::{NonZeroU64, NonZeroU32}, array, any::TypeId};

use crate::lib::{network::Network, activations::SIGMOID, sirmodel::SIRModel};
use lib::{person::Personstate, wgpuInit::{self, WgpuInit}};


pub mod lib;

//pub async fn run(event_loop: EventLoop<()>, window: Window) {
    // input is a horizontal matrix [1,3,7,19,18.2874]
//    let input = [[1 as f32,3 as f32,7 as f32, 19 as f32, 18.2874]];

    // second arr is a vertical matrix  [1.345  ]
    //                                  [2.2    ]
    //                                  [74     ]
    //                                  [20     ]
    //                                  [18.2874]
//    let secondArr = [[1.345],[2.2] ,[74 as f32], [20 as f32], [18.2874]];

    

//    let mut resVec = vec![vec![0.0 as f32;input.len()];secondArr[0].len()];


    // Should be [rowCountA, colCountB]
//    let dimensions: [u32; 2] = [input.len() as u32,secondArr[0].len() as u32];
    
//    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{ backends: wgpu::Backends::PRIMARY, dx12_shader_compiler: wgpu::Dx12Compiler::Fxc});
//    let surface = unsafe { instance.create_surface(&window)}.unwrap();
//    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
//        power_preference: wgpu::PowerPreference::default(),
//        compatible_surface: Some(&surface),
//        force_fallback_adapter: false,
//    })
//    .await
//    .expect("Failed to find an appropriate adapter");

//    let ( device , queue) = adapter.request_device(
//        &wgpu::DeviceDescriptor {
//            label: None,
//            features: wgpu::Features::empty(),
//            limits: wgpu::Limits::default(),
//        },
//        None,
//    ).await.unwrap();

//    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//        label: None,
//        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
//    });
    

//    let pipelineDescriptor = wgpu::ComputePipelineDescriptor {
//        label: Some("doubling compute pipeline"),
//        layout: None,
//        module: &shader,
//        entry_point: "matrixMult"
//    };

//    let pipeline = device.create_compute_pipeline( &pipelineDescriptor);


//    let workBufferDescriptor = wgpu::BufferDescriptor {
//        label: Some("Work Buffer"),
//        size: bytemuck::bytes_of(&input).len() as u64,
//        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
//        mapped_at_creation: false
//    };

    //let workBuffer = device.create_buffer(& workBufferDescriptor);
    //queue.write_buffer(&workBuffer,0, bytemuck::bytes_of(&input));

//    let workBuffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//        label: Some("Work Buffer"),
//        contents: bytemuck::cast_slice(&input),
//        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
//    });

//    let secondBuffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//        label: Some("Second Buffer"),
//        contents: bytemuck::cast_slice(&secondArr),
//        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
//    });

//    let dimensionBuffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//        label: Some("Dimension Buffer"),
//        contents: bytemuck::cast_slice(&dimensions),
//        usage: BufferUsages::UNIFORM
//    });

//    let resultBufferDescriptor = wgpu::BufferDescriptor {
//        label: Some("Result Buffer"),
//        size: bytemuck::bytes_of(&res).len() as u64,
//        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
//        mapped_at_creation: false
//    };

//    let resultBuffer = device.create_buffer(&resultBufferDescriptor);

//    let bindGroupLayout = pipeline.get_bind_group_layout(0);
    

//    let bindGroupDescriptor = wgpu::BindGroupDescriptor {
//        label: Some("Bindgroup for work buffer"),
//        layout: &bindGroupLayout,
//        entries: & [
//            BindGroupEntry {binding: 0, resource: workBuffer.as_entire_binding()},
//            BindGroupEntry {binding: 1, resource: secondBuffer.as_entire_binding()},
//            BindGroupEntry {binding: 2, resource: dimensionBuffer.as_entire_binding()}
//        ]
//    };

//    let bindGroup = device.create_bind_group(&bindGroupDescriptor);
    

//    let commandEncoderDescriptor = wgpu::CommandEncoderDescriptor {
//        label: Some("doubling encoder")
//    };

//    let mut encoder = device.create_command_encoder(&commandEncoderDescriptor);
    
//    let computePassDescriptor = wgpu::ComputePassDescriptor {
//        label: Some("Doubling compute pass")
//    };
//    {
//        let mut pass = encoder.begin_compute_pass(&computePassDescriptor);

//        pass.set_pipeline(&pipeline);
//        pass.set_bind_group(0, &bindGroup, &[]);
//        pass.dispatch_workgroups(input[0].len() as u32, secondArr.len() as u32, 3);
//    }

//    encoder.copy_buffer_to_buffer(&workBuffer, 0, &resultBuffer, 0, resultBuffer.size());
    
//    queue.submit(iter::once(encoder.finish()));

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

//    let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
//    let slice = resultBuffer.slice(..);
//    slice.map_async(wgpu::MapMode::Read, move |result| {
//        tx.send(result).unwrap();
//    });
    // wait for the GPU to finish
//    device.poll(wgpu::Maintain::Wait);

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
        
    

//}

#[tokio::main] 
async fn main() {
    
    println!("Hello, world!");

    let inputs = vec![
        vec![0.0,0.0],
        vec![0.0,1.0],
        vec![1.0,0.0],
        vec![1.0,1.0],
    ];

    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];
    
    let mut network = Network::new(vec![2,5,7,1],0.2,SIGMOID);
    network.train(inputs, targets, 1000);

    println!("0 and 0: {:?}", network.feed_forward(vec![0.0,0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0,1.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0,0.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0,1.0]));

    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{ backends: wgpu::Backends::PRIMARY, dx12_shader_compiler: wgpu::Dx12Compiler::Fxc, flags: wgpu::InstanceFlags::all(), gles_minor_version: wgpu::Gles3MinorVersion::Automatic });
    let surface = unsafe { instance.create_surface(&window)}.unwrap();
    window.set_title("Tringle");
    env_logger::init();
    let size = &window.inner_size();

    let wgpuinit = WgpuInit::new(size.clone(),instance,surface).await;

    let days = 100;

    let mut simulation = SIRModel::new(60000, 1.4,7,5,1.0,0.05,1000.0,1000.0,10.0,100.0,days, wgpuinit);
    simulation.runSim().await;

    let mut t: usize = 0;
    let mut simulated = false;
    let mut sim = false;

     
    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested => control_flow.exit(),
                winit::event::WindowEvent::RedrawRequested => {
                    if simulated {
                        simulation.newFrame(t);
                        if t <days {
                            t = t+1
                        } else {
                            t = 0;
                        }
                    } 
                    
                },
                _ => {}
            }
            
        },
        _ => {}
    });
    
    
    /*
    event_loop.run(move |event, control_flow| {
        *control_flow = ControlFlow::Wait;

        tokio::spawn(handle_event(event, control_flow));

        if *control_flow == ControlFlow::Exit {
            return;
        }
    });
    */
}
/* 
async fn handle_event(event: Event<()>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            // Perform cleanup or any other necessary tasks
            *control_flow = ControlFlow::Exit;
        }
        winit::event::WindowEvent::RedrawRequested => {
            if sim {
                simulation.runSim().await;
                sim = true;
            } else {
                if simulated {
                    simulation.newFrame(t);
                    if t <days {
                        t = t+1
                    } else {
                        t = 0;
                    }
                } 
            }
        }
        // Handle other events asynchronously
        _ => {
            // Your asynchronous logic here
        }
    }
}
*/