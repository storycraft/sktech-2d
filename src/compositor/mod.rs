/*
 * Created on Mon Jul 03 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod surface;

use std::borrow::Cow;

use wgpu::{
    BufferUsages, CommandEncoder, CompareFunction, DepthBiasState, DepthStencilState, Device,
    LoadOp, MultisampleState, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StencilFaceState, StencilState,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

use super::context::{DrawContext, RenderContext};

use crate::{
    buffer::stream::BufferStream,
    pipeline::RenderPipelineData,
    queue::RenderNodeQueue,
    store::{BackendFnStore, RenderData},
};

use crate::{node::DrawNode, screen::ScreenRect};

pub struct Compositor {

    queue: RenderNodeQueue,

    data: RenderData,

    pipeline_data: RenderPipelineData,

    depth_texture_size: (u32, u32),
    depth_texture: Option<TextureView>,

    vertex_stream: BufferStream<'static>,
    index_stream: BufferStream<'static>,
}

impl Compositor {
    pub const DEPTH_TEXTURE_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn new(texture_format: TextureFormat, multi_sample: Option<MultisampleState>) -> Self {
        let vertex_stream = BufferStream::new(
            Some(Cow::from("Compositor vertex stream buffer")),
            BufferUsages::VERTEX,
        );
        let index_stream = BufferStream::new(
            Some(Cow::from("Compositor index stream buffer")),
            BufferUsages::INDEX,
        );

        Self {
            queue: RenderNodeQueue::new(),

            data: RenderData::new(),

            pipeline_data: Self::create_pipeline_data(texture_format, multi_sample),

            depth_texture_size: Default::default(),
            depth_texture: None,

            vertex_stream,
            index_stream,
        }
    }

    const fn create_pipeline_data(
        texture_format: TextureFormat,
        multi_sample: Option<MultisampleState>,
    ) -> RenderPipelineData {
        RenderPipelineData {
            texture_format,
            depth_stencil: Some(DepthStencilState {
                format: Self::DEPTH_TEXTURE_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            multi_sample,
        }
    }

    pub const fn pipeline_data(&self) -> &RenderPipelineData {
        &self.pipeline_data
    }

    fn update_depth_stencil(&mut self, device: &Device, width: u32, height: u32) {
        self.depth_texture = Some(
            device
                .create_texture(&TextureDescriptor {
                    label: Some("StoryboardRenderer depth texture"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: Self::DEPTH_TEXTURE_FORMAT,
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&TextureViewDescriptor::default()),
        );
    }

    pub fn prepare<'a>(
        &mut self,
        backend_store: BackendFnStore,
        encoder: &mut CommandEncoder,
        screen: ScreenRect,
        draw_nodes: impl ExactSizeIterator<Item = &'a dyn DrawNode>,
    ) {
        if draw_nodes.len() == 0 || screen.is_none() {
            return;
        }

        if self.depth_texture_size != screen.size() {
            self.update_depth_stencil(backend_store.device(), screen.width, screen.height);

            self.depth_texture_size = screen.size();
        }

        let mut draw_context = DrawContext {
            store: self.data.as_ref(backend_store, &self.pipeline_data),

            encoder,
            queue: &mut self.queue,

            screen,
            vertex_stream: &mut self.vertex_stream,
            index_stream: &mut self.index_stream,
        };

        let total = draw_nodes.len() as f32;
        for (i, drawable) in draw_nodes.enumerate() {
            drawable.prepare(&mut draw_context, 1.0_f32 - ((1.0_f32 + i as f32) / total));
        }
    }

    pub fn draw(
        &mut self,
        backend_store: BackendFnStore,
        encoder: &mut CommandEncoder,
        color_attachments: &[Option<RenderPassColorAttachment>],
    ) {
        if self.queue.is_empty() {
            return;
        }

        let depth_attachment = RenderPassDepthStencilAttachment {
            view: self.depth_texture.as_ref().unwrap(),
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        };

        let device = backend_store.device();
        let queue = backend_store.queue();

        let vertex_stream = self.vertex_stream.finish(device, queue);
        let index_stream = self.index_stream.finish(device, queue);

        {
            let ctx = RenderContext {
                store: self.data.as_ref(backend_store, &self.pipeline_data),
                vertex_stream,
                index_stream,
            };

            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("StoryboardRenderer render pass"),
                color_attachments,
                depth_stencil_attachment: Some(depth_attachment),
            });

            self.queue.render(&ctx, &mut pass);
        }

        self.queue.clear();
    }
}
