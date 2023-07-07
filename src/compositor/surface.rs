/*
 * Created on Mon Jul 03 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{fmt::Debug, iter};

use wgpu::{
    self, Adapter, Color, CommandEncoderDescriptor, CompositeAlphaMode, LoadOp, Operations,
    PresentMode, RenderPassColorAttachment, Surface, SurfaceError, TextureUsages,
    TextureViewDescriptor,
};

use crate::{node::DrawNode, screen::ScreenRect, store::BackendFnStore};

use super::Compositor;

pub struct SurfaceCompositor {
    surface: Surface,

    configuration: (SurfaceConfiguration, bool),

    compositor: Compositor,
}

impl SurfaceCompositor {
    pub fn new(adapter: &Adapter, surface: Surface, configuration: SurfaceConfiguration) -> Self {
        let capabilites = surface.get_capabilities(adapter);

        Self {
            surface,

            configuration: (configuration, true),

            compositor: Compositor::new(capabilites.formats[0], None),
        }
    }

    pub const fn configuration(&self) -> SurfaceConfiguration {
        self.configuration.0
    }

    pub fn set_configuration(&mut self, configuration: SurfaceConfiguration) {
        self.configuration = (configuration, true);
    }

    pub fn render<'a>(
        &mut self,
        store: BackendFnStore,
        draw_nodes: impl ExactSizeIterator<Item = &'a dyn DrawNode>,
    ) -> Result<bool, SurfaceError> {
        if draw_nodes.len() == 0 || self.configuration.0.screen.is_none() {
            return Ok(false);
        }

        if self.configuration.1 {
            let configuration = &self.configuration.0;

            self.surface.configure(
                store.device(),
                &wgpu::SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    format: self.compositor.pipeline_data().texture_format,
                    width: configuration.screen.width,
                    height: configuration.screen.height,
                    present_mode: configuration.present_mode,
                    alpha_mode: configuration.alpha_mode,
                    view_formats: vec![],
                },
            );

            self.configuration.1 = false;
        }
        let surface_texture = self.surface.get_current_texture()?;

        let screen = self.configuration.0.screen;

        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = store
            .device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("SurfaceCompositor"),
            });

        self.compositor
            .prepare(store.clone(), &mut encoder, screen, draw_nodes);

        self.compositor.draw(
            store.clone(),
            &mut encoder,
            &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            })],
        );

        store.queue().submit(iter::once(encoder.finish()));
        surface_texture.present();

        Ok(true)
    }

    pub fn into_inner(self) -> Surface {
        self.surface
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceConfiguration {
    pub present_mode: PresentMode,
    pub screen: ScreenRect,
    pub alpha_mode: CompositeAlphaMode,
}
