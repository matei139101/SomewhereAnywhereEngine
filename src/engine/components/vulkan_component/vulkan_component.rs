use glam::Vec3;

use crate::engine::{
    components::vulkan_component::vulkan_events::{CreateVulkanInstanceEvent, VulkanDrawEvent},
    event_bus::event_bus::EventBus,
    vulkan::vulkan_container::{self, VulkanContainer},
};
use std::sync::{Arc, Mutex};

pub struct VulkanComponent {
    vulkan_container: Option<Arc<Mutex<VulkanContainer>>>,
    event_bus_ptr: Arc<EventBus>,
}

impl VulkanComponent {
    pub fn new(event_bus_ptr: Arc<EventBus>) -> Arc<Mutex<VulkanComponent>> {
        let vulkan_component = Arc::new(Mutex::new(VulkanComponent {
            vulkan_container: Default::default(),
            event_bus_ptr,
        }));

        VulkanComponent::observe_events(vulkan_component.clone());

        vulkan_component
    }

    pub fn observe_events(self_ptr: Arc<Mutex<VulkanComponent>>) {
        let bus_arc = {
            let this = self_ptr.lock().unwrap();
            this.event_bus_ptr.clone()
        };

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<CreateVulkanInstanceEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<CreateVulkanInstanceEvent>() {
                    if let Ok(mut vulkan) = self_ptr_clone.lock() {
                        vulkan.create_vulkan_container(event.vulkan_container.clone());
                    }
                }
            }));

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<VulkanDrawEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<VulkanDrawEvent>() {
                    if let Ok(mut vulkan) = self_ptr_clone.lock() {
                        vulkan.draw_frame(&event.viewport_location, &event.viewport_rotation);
                        let _ = event
                            .confirmation_sender
                            .lock()
                            .unwrap()
                            .take()
                            .unwrap()
                            .send(());
                    }
                }
            }));
    }

    fn create_vulkan_container(&mut self, vulkan_container: Arc<Mutex<VulkanContainer>>) {
        self.vulkan_container = Some(vulkan_container);
        println!(
            "Vulkan container is something: {:?}",
            self.vulkan_container.is_some()
        )
    }

    fn draw_frame(&mut self, viewport_location: &Vec3, viewport_rotation: &Vec3) {
        self.vulkan_container
            .as_mut()
            .unwrap()
            .lock()
            .unwrap()
            .draw_frame(viewport_location, viewport_rotation);
    }

    /*:w

    fn resize_viewport(&mut self, event_info: &ViewportResizeInfo) {
        self.vulkan_container
            .resize_viewport(&event_info.viewport_information);
    }

    fn create_vulkan_object(&mut self, object_info: &ObjectCreateInfo) {
        self.vulkan_container.create_vulkan_object(
            object_info.object_id,
            object_info.vertices.clone(),
            object_info.object_transform.clone(),
            &object_info.texture_path.clone(),
        );
    }

    fn delete_vulkan_object(&mut self, object_id: &usize) {
        self.vulkan_container.delete_vulkan_object(*object_id);
    }
    */
}
