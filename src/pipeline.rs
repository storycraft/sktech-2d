/*
 * Created on Thu Jul 06 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use wgpu::{TextureFormat, MultisampleState, DepthStencilState};

#[derive(Debug, Clone)]
pub struct RenderPipelineData {
    pub texture_format: TextureFormat,
    pub depth_stencil: Option<DepthStencilState>,
    pub multi_sample: Option<MultisampleState>,
}

impl RenderPipelineData {
    pub fn depth_stencil_read_only(&self) -> Option<DepthStencilState> {
        self.depth_stencil.clone().map(|mut depth_stencil| {
            depth_stencil.depth_write_enabled = false;
            depth_stencil
        })
    }
}
