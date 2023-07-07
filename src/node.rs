/*
 * Created on Mon Jul 03 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use wgpu::RenderPass;

use crate::context::{DrawContext, RenderContext};

pub trait DrawNode: Send {
    fn prepare(&self, ctx: &mut DrawContext, depth: f32);
}

pub trait RenderNode {
    fn render_opaque<'rpass>(
        &'rpass self,
        ctx: &RenderContext<'rpass>,
        pass: &mut RenderPass<'rpass>,
    );

    fn render_transparent<'rpass>(
        &'rpass self,
        ctx: &RenderContext<'rpass>,
        pass: &mut RenderPass<'rpass>,
    );
}
