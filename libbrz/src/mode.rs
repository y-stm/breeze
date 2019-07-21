use crate::action;
use crate::idx::*;
use crate::state::State;
use crate::Key;
use std::path::PathBuf;

mod command;
mod find;
mod goto;
mod insert;
mod normal;

pub use self::command::Command;
pub use self::find::Find;
pub use self::goto::Goto;
pub use self::insert::Insert;
pub use self::normal::Normal;

pub use crate::render::{self, Rect, Renderer};

// TODO: mode should render itself, ha!
pub trait Mode {
    fn name(&self) -> &str;
    fn name4(&self) -> &str {
        &self.name()[..4]
    }
    fn cmd_string(&self) -> Option<String> {
        None
    }
    fn available_actions(&self) -> &action::Map {
        action::no_actions()
    }

    fn on_enter(&mut self, _state: &State) {}

    fn handle(&mut self, state: &mut State, key: Key);

    fn render(&self, state: &State, render: &mut dyn Renderer) {
        let _ = default_render(self, state, render);
    }
}

fn default_render_split_status_rect(render: &mut dyn Renderer) -> (Rect, Rect) {
    let total_rect = render.dimensions_rect();
    total_rect.split_horizontaly_at(-1)
}

fn default_render_status(
    mode: &(impl Mode + ?Sized),
    mut render: &mut dyn Renderer,
    status_rect: Rect,
) {
    let style = render.color_map().default;
    let mut status_view = status_rect.to_renderer(&mut render);
    status_view.print(
        render::Coord {
            x: status_rect.dimensions.x.saturating_sub(4),
            y: 0,
        },
        mode.name4(),
        style,
    );
}

fn default_render(
    mode: &(impl Mode + ?Sized),
    state: &State,
    mut render: &mut dyn Renderer,
) -> (Rect, Rect) {
    let (buffer_rect, status_rect) = default_render_split_status_rect(render);
    state.render_buffer(&mut buffer_rect.to_renderer(&mut render));

    default_render_status(mode, render, status_rect);

    (buffer_rect, status_rect)
}
