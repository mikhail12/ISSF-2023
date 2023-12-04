

use wgpu::{Device, Instance, Surface, Adapter, Queue, ShaderModule, ComputePipeline, BufferUsages, util::DeviceExt, BindGroupEntry, BufferDescriptor};
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}, dpi::PhysicalSize};

use std::{borrow::Cow, iter, num::{NonZeroU64, NonZeroU32}, array, any::TypeId};



pub struct WgpuInit {
    pub size: PhysicalSize<u32>,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue
}

impl WgpuInit {

    pub async fn new<'a>( size: PhysicalSize<u32>, instance: Instance, surface: Surface) -> WgpuInit{
        

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Option::None,
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

        WgpuInit { size, instance, surface, adapter, device, queue}
    }

    pub fn newFrame(&mut self, posx: Vec<f32>, posy: Vec<f32>, inf: Vec<u32>) {
        let mut posxvec: Vec<f32> = posx;
        let mut posyvec: Vec<f32> = posy;
        let mut infvec: Vec<u32> = inf;

        let infBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Infected or not array Buffer"),
            contents: u32_vector_to_bytes(&infvec),
            //contents: bytemuck::cast_slice(&posyvec),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });


        let xPosBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("X Positions array Buffer"),
            contents: f32_vector_to_bytes(&posxvec),
            //contents: bytemuck::cast_slice(&posxvec),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_SRC | BufferUsages::COPY_DST 
        });

        let yPosBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Y Positions array Buffer"),
            contents: f32_vector_to_bytes(&posyvec),
            //contents: bytemuck::cast_slice(&posyvec),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });


        let surface_capabilities = self.surface.get_capabilities(&self.adapter);
        let format = surface_capabilities.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![]
        };

        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Point Frame Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("pointShader.wgsl")))
        });

        let binding = [Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE
                    }),
                    write_mask: wgpu::ColorWrites::ALL
                })];

        let pipelineDescriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline Descriptor"),
            layout: None,
            vertex: wgpu::VertexState { 
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &binding
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        };

        let pipeline = self.device.create_render_pipeline(&pipelineDescriptor);

        let infBufBindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: NonZeroU32::new(infvec.len() as u32)
        };
        let xPosBufBindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: NonZeroU32::new(posxvec.len() as u32)
        };
        let yPosBufBindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: NonZeroU32::new(posyvec.len() as u32)
        };

        let bindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout descriptor"),
            entries: &[infBufBindGroupLayoutEntry, xPosBufBindGroupLayoutEntry, yPosBufBindGroupLayoutEntry]
        };
        let bindGroupLayout = self.device.create_bind_group_layout(&bindGroupLayoutDescriptor);

        //let mut bindGroupLayout = pipeline.get_bind_group_layout(1);


        let bindGroupDescriptor = wgpu::BindGroupDescriptor {
            label: Some("Bindgroup for work buffer"),
            layout: &bindGroupLayout,
            entries: & [
                BindGroupEntry {binding: 0, resource: infBuffer.as_entire_binding()},
                BindGroupEntry {binding: 1, resource: xPosBuffer.as_entire_binding()},
                BindGroupEntry {binding: 2, resource: yPosBuffer.as_entire_binding()},
            ]
        };

        let bindGroup = self.device.create_bind_group(&bindGroupDescriptor);
    

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: Some("Render Encoder")});
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.062, b: 0.08, a: 1.0 }),
                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            });
            rpass.set_pipeline(&pipeline);
            rpass.set_bind_group(0, &bindGroup, &[]);
            frame.present();
        }
    }

    pub async fn checkInf(&mut self, posx: Vec<f32>, posy: Vec<f32>, inf: Vec<u32>, infRad: f32) -> Vec<u32> {
        
        
        let mut posxvec: Vec<f32> = posx;
        let mut posyvec: Vec<f32> = posy;
        let mut infvec: Vec<u32> = inf;

        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("checkinfshader.wgsl"))),
        });

        let pipelineDescriptor = wgpu::ComputePipelineDescriptor {
            label: Some("Move Pos Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "checkInf"
        };

        let pipeline = self.device.create_compute_pipeline( &pipelineDescriptor);

        let infRadBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Infectious Radius Buffer"),
            contents: bytemuck::bytes_of(&infRad),
            //contents: bytemuck::cast_slice(&edges[0]),
            usage: BufferUsages::UNIFORM 
        });

        let infBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Infected or not array Buffer"),
            contents: u32_vector_to_bytes(&infvec),
            //contents: bytemuck::cast_slice(&posyvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });

        let xPosBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("X Positions array Buffer"),
            contents: f32_vector_to_bytes(&posxvec),
            //contents: bytemuck::cast_slice(&posxvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST 
        });

        let yPosBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Y Positions array Buffer"),
            contents: f32_vector_to_bytes(&posyvec),
            //contents: bytemuck::cast_slice(&posyvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });

        let resInfBuffer = self.device.create_buffer(& BufferDescriptor {
            label: Some("Infected or not Results array Buffer"),
            size: u32_vector_to_bytes(&infvec).len() as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let bindGroupLayout = pipeline.get_bind_group_layout(0);

        let bindGroupDescriptor = wgpu::BindGroupDescriptor {
            label: Some("Bindgroup for work buffer"),
            layout: &bindGroupLayout,
            entries: & [
                BindGroupEntry {binding: 0, resource: infRadBuffer.as_entire_binding()},
                BindGroupEntry {binding: 1, resource: infBuffer.as_entire_binding()},
                BindGroupEntry {binding: 2, resource: xPosBuffer.as_entire_binding()},
                BindGroupEntry {binding: 3, resource: yPosBuffer.as_entire_binding()},
            ]
        };

        let bindGroup = self.device.create_bind_group(&bindGroupDescriptor);
    

        let commandEncoderDescriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Check infected command encoder")
        };

        let mut encoder = self.device.create_command_encoder(&commandEncoderDescriptor);
        
        let computePassDescriptor = wgpu::ComputePassDescriptor {
            label: Some("Check Infected compute pass"),
            timestamp_writes: Option::None
        };
        {
            let mut pass = encoder.begin_compute_pass(&computePassDescriptor);

            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bindGroup, &[]);
            pass.dispatch_workgroups(infvec.len() as u32, infvec.len() as u32, 1);
        }

        encoder.copy_buffer_to_buffer(&infBuffer, 0, &resInfBuffer, 0, u32_vector_to_bytes(&infvec).len() as u64);
        
        self.queue.submit(iter::once(encoder.finish()));


        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();

        let infSlice = resInfBuffer.slice(..);
        

        infSlice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });

        self.device.poll(wgpu::Maintain::Wait);
        

        match rx.receive().await {
            Some(Ok(())) => {
                let data = infSlice.get_mapped_range();
                let res = data.chunks_exact(4).map(|b| u32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<u32>>();
                infvec = res;
                drop(data);
                resInfBuffer.unmap();
            }
            _ => println!("Something went wrong"),
        }

        infvec
    }

    pub async fn moveCol(&mut self, posx: Vec<f32>, posy: Vec<f32>, velx: Vec<f32>, vely: Vec<f32>, edges: [f32;2]) -> [Vec<f32>;4] {
        
        
        let mut posxvec: Vec<f32> = posx;
        let mut posyvec: Vec<f32> = posy;
        let mut velxvec: Vec<f32> = velx;
        let mut velyvec: Vec<f32> = vely;

        //println!("Bytes are thiss {:?}  wooooo",f32_vector_to_bytes(&posyvec));
        
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("moveshader.wgsl"))),
        });

        let pipelineDescriptor = wgpu::ComputePipelineDescriptor {
            label: Some("Move Pos Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "movePosChange"
        };

        let pipeline = self.device.create_compute_pipeline( &pipelineDescriptor);

        let xEdgeBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("X Edge Buffer"),
            contents: bytemuck::bytes_of(&edges[0]),
            //contents: bytemuck::cast_slice(&edges[0]),
            usage: BufferUsages::UNIFORM 
        });


        let yEdgeBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Y Edge Buffer"),
            contents: bytemuck::bytes_of(&edges[1]),
            //contents: bytemuck::cast_slice(&edges[1]),
            usage: BufferUsages::UNIFORM 
        });

        //println!("{:?} sdfbaisdfbu", f32_vector_to_bytes(&posxvec));
    
        let xPosBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("X Positions array Buffer"),
            contents: f32_vector_to_bytes(&posxvec),
            //contents: bytemuck::cast_slice(&posxvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });


        let yPosBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Y Positions array Buffer"),
            contents: f32_vector_to_bytes(&posyvec),
            //contents: bytemuck::cast_slice(&posyvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });

        let xVelBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("X Velocities array Buffer"),
            contents: f32_vector_to_bytes(&velxvec),
            //contents: bytemuck::cast_slice(&velxvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });

        let yVelBuffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Y Velocities array Buffer"),
            contents: f32_vector_to_bytes(&velyvec),
            //contents: bytemuck::cast_slice(&velxvec),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST
        });

        let bytesNum = f32_vector_to_bytes(&posxvec).len() as u64;

        let resXPosBuffer = self.device.create_buffer(& BufferDescriptor {
            label: Some("X Positions Results array Buffer"),
            size: f32_vector_to_bytes(&posxvec).len() as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let resYPosBuffer = self.device.create_buffer(& BufferDescriptor {
            label: Some("Y Positions Redsults array Buffer"),
            size: f32_vector_to_bytes(&posyvec).len() as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let resXVelBuffer = self.device.create_buffer(& BufferDescriptor {
            label: Some("X Velocities Results array Buffer"),
            size: f32_vector_to_bytes(&velxvec).len() as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let resYVelBuffer = self.device.create_buffer(& BufferDescriptor {
            label: Some("Y Velocities Results array Buffer"),
            size: f32_vector_to_bytes(&velyvec).len() as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let bindGroupLayout = pipeline.get_bind_group_layout(0);

        let bindGroupDescriptor = wgpu::BindGroupDescriptor {
            label: Some("Bindgroup for work buffer"),
            layout: &bindGroupLayout,
            entries: & [
                BindGroupEntry {binding: 0, resource: xEdgeBuffer.as_entire_binding()},
                BindGroupEntry {binding: 1, resource: yEdgeBuffer.as_entire_binding()},
                BindGroupEntry {binding: 2, resource: xPosBuffer.as_entire_binding()},
                BindGroupEntry {binding: 3, resource: yPosBuffer.as_entire_binding()},
                BindGroupEntry {binding: 4, resource: xVelBuffer.as_entire_binding()},
                BindGroupEntry {binding: 5, resource: yVelBuffer.as_entire_binding()}
            ]
        };

        let bindGroup = self.device.create_bind_group(&bindGroupDescriptor);
    

        let commandEncoderDescriptor = wgpu::CommandEncoderDescriptor {
            label: Some("move command encoder")
        };

        let mut encoder = self.device.create_command_encoder(&commandEncoderDescriptor);
        
        let computePassDescriptor = wgpu::ComputePassDescriptor {
            label: Some("move compute pass"),
            timestamp_writes: Option::None
        };
        {
            let mut pass = encoder.begin_compute_pass(&computePassDescriptor);

            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bindGroup, &[]);
            pass.dispatch_workgroups(posxvec.len() as u32, 1, 1);
        }
        
        encoder.copy_buffer_to_buffer(&xPosBuffer, 0, &resXPosBuffer, 0, bytesNum);
        encoder.copy_buffer_to_buffer(&yPosBuffer, 0, &resYPosBuffer, 0, bytesNum);
        encoder.copy_buffer_to_buffer(&xVelBuffer, 0, &resXVelBuffer, 0, bytesNum);
        encoder.copy_buffer_to_buffer(&yVelBuffer, 0, &resYVelBuffer, 0, bytesNum);
        
        self.queue.submit(iter::once(encoder.finish()));


        let (xPosTx, xPosRx) = futures_intrusive::channel::shared::oneshot_channel();
        let (yPosTx, yPosRx) = futures_intrusive::channel::shared::oneshot_channel();
        let (xVelTx, xVelRx) = futures_intrusive::channel::shared::oneshot_channel();
        let (yVelTx, yVelRx) = futures_intrusive::channel::shared::oneshot_channel();

        let xPosSlice = resXPosBuffer.slice(..);
        let yPosSlice = resYPosBuffer.slice(..);
        let xVelSlice = resXVelBuffer.slice(..);
        let yVelSlice = resYVelBuffer.slice(..);

        xPosSlice.map_async(wgpu::MapMode::Read, move |result| {
            xPosTx.send(result).unwrap();
        });
        yPosSlice.map_async(wgpu::MapMode::Read, move |result| {
            yPosTx.send(result).unwrap();
        });
        
        xVelSlice.map_async(wgpu::MapMode::Read, move |result| {
            xVelTx.send(result).unwrap();
        });
        yVelSlice.map_async(wgpu::MapMode::Read, move |result| {
            yVelTx.send(result).unwrap();
        });

        self.device.poll(wgpu::Maintain::Wait);
        

        match xPosRx.receive().await {
            Some(Ok(())) => {
                let data = xPosSlice.get_mapped_range();
                let res = data.chunks_exact(4).map(|b| f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<f32>>();
                posxvec = res;
                drop(data);
                resXPosBuffer.unmap();
            }
            _ => println!("Something went wrong"),
        }

        match yPosRx.receive().await {
            Some(Ok(())) => {
                let data = yPosSlice.get_mapped_range();
                let res = data.chunks_exact(4).map(|b| f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<f32>>();
                posyvec = res;
                drop(data);
                resYPosBuffer.unmap();
            }
            _ => println!("Something went wrong"),
        }

        match xVelRx.receive().await {
            Some(Ok(())) => {
                let data = xVelSlice.get_mapped_range();
                let res = data.chunks_exact(4).map(|b| f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<f32>>();
                velxvec = res;
                drop(data);
                resXVelBuffer.unmap();
            }
            _ => println!("Something went wrong"),
        }


        match yVelRx.receive().await {
            Some(Ok(())) => {
                let data = yVelSlice.get_mapped_range();
                let res = data.chunks_exact(4).map(|b| f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<f32>>();
                velyvec = res;
                drop(data);
                resYVelBuffer.unmap();
            }
            _ => println!("Something went wrong"),
        }

        [posxvec, posyvec,velxvec, velyvec]
    }
}

fn f32_vector_to_bytes(data: &Vec<f32>) -> &[u8] {
    let data_as_slice: &[f32] = &data;
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            data_as_slice.as_ptr() as *const u8,
            data_as_slice.len() * std::mem::size_of::<f32>(),
        )
    };
    bytes
}

fn u32_vector_to_bytes(data: &Vec<u32>) -> &[u8] {
    let data_as_slice: &[u32] = &data;
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            data_as_slice.as_ptr() as *const u8,
            data_as_slice.len() * std::mem::size_of::<u32>(),
        )
    };
    bytes
}