use crate::engine::{
    event_bus::{
        event_bus::EventBus,
        events::VulkanEvents::{VulkanDrawEvent, VulkanViewportResizeEvent},
    },
    vulkan::vulkan_container::VulkanContainer,
};
use std::sync::{Arc, Mutex};

pub struct VulkanComponent {
    vulkan_container: VulkanContainer,
}

impl VulkanComponent {
    pub fn new(
        vulkan_container: VulkanContainer,
        event_bus: Arc<Mutex<EventBus>>,
    ) -> Arc<Mutex<Self>> {
        let vulkan_component = Arc::new(Mutex::new(VulkanComponent { vulkan_container }));
        VulkanComponent::subscribe_to_events(&vulkan_component, event_bus.clone());

        vulkan_component
    }

    fn draw_frame(&mut self, event_info: &VulkanDrawEvent) {
        self.vulkan_container
            .draw_frame(event_info.viewport_location, event_info.viewport_rotation);
    }

    fn resize_viewport(&mut self, event_info: &VulkanViewportResizeEvent) {
        self.vulkan_container
            .resize_viewport(&event_info.viewport_information);
    }

    fn subscribe_to_events(self_ptr: &Arc<Mutex<Self>>, event_bus: Arc<Mutex<EventBus>>) {
        let weak_self = Arc::downgrade(self_ptr);

        // Much cleaner! No more <Self, PlayerMoved, _>
        event_bus.lock().unwrap().subscribe({
            let weak_self = weak_self.clone();
            move |event: &VulkanDrawEvent| {
                if let Some(strong_self) = weak_self.upgrade() {
                    if let Ok(mut vulkan_component) = strong_self.lock() {
                        vulkan_component.draw_frame(event);
                    }
                }
            }
        });

        event_bus.lock().unwrap().subscribe({
            let weak_self = weak_self.clone();
            move |event: &VulkanViewportResizeEvent| {
                if let Some(strong_self) = weak_self.upgrade() {
                    if let Ok(mut vulkan_component) = strong_self.lock() {
                        vulkan_component.resize_viewport(event);
                    }
                }
            }
        });
    }
}
