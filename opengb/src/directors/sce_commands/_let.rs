use crate::directors::{sce_director::SceCommand, sce_state::SceState};
use imgui::Ui;
use radiance::scene::SceneManager;

#[derive(Clone)]
pub struct SceCommandLet {
    var: i16,
    value: i32,
}

impl SceCommand for SceCommandLet {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        state
            .ext_mut()
            .insert(self.var.to_string(), Box::new(self.value));
        true
    }
}

impl SceCommandLet {
    pub fn new(var: i16, value: i32) -> Self {
        Self { var, value }
    }
}