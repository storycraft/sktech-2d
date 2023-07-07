/*
 * Created on Mon Jul 03 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use wgpu::CommandEncoder;

use crate::{
    buffer::stream::{BufferStream, StreamBuffer},
    queue::RenderNodeQueue,
    screen::ScreenRect,
    store::RenderFnStore,
};

/// [DrawContext] contains reference to backend, resources store, and stream for component data preparing
pub struct DrawContext<'a, 'encoder> {
    pub store: RenderFnStore<'a>,

    pub encoder: &'encoder mut CommandEncoder,
    pub queue: &'encoder mut RenderNodeQueue,

    pub screen: ScreenRect,

    pub vertex_stream: &'a mut BufferStream<'static>,
    pub index_stream: &'a mut BufferStream<'static>,
}

/// [RenderContext] contains gpu device and stream for component rendering
pub struct RenderContext<'a> {
    pub store: RenderFnStore<'a>,

    pub vertex_stream: StreamBuffer<'a>,
    pub index_stream: StreamBuffer<'a>,
}
