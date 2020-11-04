use std::sync::Arc;


struct Instance {
    context: Arc<egui::Context>,
    raw_input: egui::RawInput,
}

impl Instance {
    fn new(size: egui::Vec2, scale_factor: f32) -> Self {
        let context = egui::Context::new();
        let raw_input = egui::RawInput {
            mouse_down: false,
            mouse_pos: None,
            scroll_delta: Default::default(),
            screen_size: size,
            pixels_per_point: Some(scale_factor),
            time: ,
            events: vec![],
        };

        Self {
            context
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
