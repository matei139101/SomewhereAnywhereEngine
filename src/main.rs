use std::{any::Any, thread, time::Duration};
use tokio::{
    self,
    runtime::Runtime,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use crate::engine::{
    app::App, components::vulkan_component::vulkan_component::VulkanComponent,
    event_bus::event_bus::EventBus,
};
mod engine;

fn main() {
    let (async_sender, async_receiver) = mpsc::unbounded_channel::<Box<dyn Any + Send + Sync>>();

    make_async_runner(async_sender.clone(), async_receiver);
    make_sync_runner(async_sender.clone());
}

fn make_async_runner(
    _async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    async_receiver: UnboundedReceiver<Box<dyn Any + Send + Sync>>,
) {
    thread::spawn(move || {
        println!("Async thread started");
        let async_runtime = tokio::runtime::Runtime::new().unwrap();

        let event_bus = EventBus::new();
        let _vulkan_component = VulkanComponent::new(event_bus.clone());

        async_runtime.block_on(async {
            EventBus::run(event_bus.clone(), async_receiver).await;
        })
    });
}

fn make_sync_runner(async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>) {
    let sync_runtime = Runtime::new().unwrap();
    println!("Synchronous runtime created");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(sync_runtime, async_sender);

    let _ = event_loop.run_app(&mut app);
}
