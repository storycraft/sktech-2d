/*
 * Created on Mon Jul 03 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenRect {
    pub x: u32,
    pub y: u32,

    pub width: u32,
    pub height: u32,

    pub scale_factor: f32,
}

impl ScreenRect {
    pub const ZERO: ScreenRect = ScreenRect::new(0, 0, 0, 0, 1.0);

    pub const fn new(x: u32, y: u32, width: u32, height: u32, scale_factor: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            scale_factor,
        }
    }

    pub const fn pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub const fn size(&self) -> (u32, u32) {
        (self.width, self.width)
    }

    pub const fn is_none(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn logical_width(&self) -> f32 {
        self.width as f32 / self.scale_factor
    }

    pub fn logical_height(&self) -> f32 {
        self.height as f32 / self.scale_factor
    }

    pub fn logical_size(&self) -> (f32, f32) {
        (self.logical_width(), self.logical_height())
    }

    pub fn to_render_pos(&self, logical_x: f32, logical_y: f32) -> (f32, f32) {
        (
            ((logical_x - self.x as f32) / self.logical_width()) * 2.0 - 1.0,
            ((logical_y - self.y as f32) / self.logical_height()) * 2.0 - 1.0,
        )
    }
}
