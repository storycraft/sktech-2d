/*
 * Created on Thu Jul 06 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use unsized_stack::UnsizedStack;
use wgpu::RenderPass;

use crate::{context::RenderContext, node::RenderNode};

#[derive(Default)]
pub struct RenderNodeQueue {
    opaque: UnsizedStack<dyn RenderNode>,
    transparent: UnsizedStack<dyn RenderNode>,
}

impl RenderNodeQueue {
    pub const fn new() -> Self {
        Self {
            opaque: UnsizedStack::new(),
            transparent: UnsizedStack::new(),
        }
    }

    pub fn push_opaque(&mut self, component: impl RenderNode + 'static) {
        self.opaque.push(component, |item| item);
    }

    pub fn push_transparent(&mut self, component: impl RenderNode + 'static) {
        self.transparent.push(component, |item| item);
    }

    pub fn len(&self) -> usize {
        self.opaque.len() + self.transparent.len()
    }

    pub fn opaque_len(&self) -> usize {
        self.opaque.len()
    }

    pub fn transparent_len(&self) -> usize {
        self.transparent.len()
    }

    pub fn is_empty(&self) -> bool {
        self.opaque.is_empty() && self.transparent.is_empty()
    }

    pub fn render<'rpass>(
        &'rpass self,
        ctx: &RenderContext<'rpass>,
        pass: &mut RenderPass<'rpass>,
    ) {
        for component in self.opaque.iter().rev() {
            component.render_transparent(ctx, pass);
        }

        for component in self.transparent.iter() {
            component.render_transparent(ctx, pass);
        }
    }

    pub fn clear(&mut self) {
        self.opaque.clear();
        self.transparent.clear();
    }
}
