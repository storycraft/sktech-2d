/*
 * Created on Thu Jul 06 2023
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use fn_store::{AtomicFnStore, LocalFnStore};
use wgpu::{Device, Queue};

use crate::pipeline::RenderPipelineData;

#[derive(Debug, Default)]
pub struct BackendData(AtomicFnStore<'static>);

impl BackendData {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn as_ref<'a>(
        &'a self,
        device: &'a Device,
        queue: &'a Queue,
    ) -> BackendFnStore<'a> {
        BackendFnStore {
            device,
            queue,
            store: &self.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct RenderData(LocalFnStore<'static>);

impl RenderData {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn as_ref<'a>(
        &'a self,
        backend: BackendFnStore<'a>,
        pipeline: &'a RenderPipelineData,
    ) -> RenderFnStore<'a> {
        RenderFnStore {
            backend,
            pipeline,
            store: &self.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackendFnStore<'a> {
    device: &'a Device,
    queue: &'a Queue,
    store: &'a AtomicFnStore<'static>,
}

impl<'a> BackendFnStore<'a> {
    pub const fn device(&self) -> &'a Device {
        self.device
    }

    pub const fn queue(&self) -> &'a Queue {
        self.queue
    }

    pub fn get<T: Sync + Send + 'static>(&self, func: impl FnOnce(&Self) -> T) -> &'a T {
        self.store.get(|| func(self))
    }
}

#[derive(Debug, Clone)]
pub struct RenderFnStore<'a> {
    backend: BackendFnStore<'a>,
    pipeline: &'a RenderPipelineData,
    store: &'a LocalFnStore<'static>,
}

impl<'a> RenderFnStore<'a> {
    pub const fn backend(&self) -> &BackendFnStore<'a> {
        &self.backend
    }

    pub const fn pipeline(&self) -> &'a RenderPipelineData {
        self.pipeline
    }

    pub fn get<T: Send + 'static>(&self, func: impl FnOnce(&Self) -> T) -> &'a T {
        self.store.get(|| func(self))
    }
}
