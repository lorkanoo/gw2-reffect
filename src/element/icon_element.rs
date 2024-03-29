use super::{Context, Icon, Render, State};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconElement {
    icon: Icon,
    size: [f32; 2],
    offset: [f32; 2],
}

impl Render for IconElement {
    fn load(&mut self) {
        self.icon.load();
    }

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut State) {
        state.with_offset(self.offset, |state| {
            if self.icon.is_active(ctx) {
                self.icon.render(ui, state.pos, self.size)
            }
        })
    }
}

impl Default for IconElement {
    fn default() -> Self {
        Self {
            icon: Icon::default(),
            size: [32.0, 32.0],
            offset: [0.0, 0.0],
        }
    }
}
