mod modules;
mod grid;

use bevy::{
    prelude::*,
    app::App,
};
use bevy_mod_picking::prelude::*;
use crate::modules::ModulesPlugin;
use crate::modules::ModuleDropped;
use crate::grid::GridPlugin;

pub struct GamePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    _Hangar,
    _Battle,
}

impl From<ListenerInput<Pointer<DragEnd>>> for ModuleDropped {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        ModuleDropped(event.target)
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins(GridPlugin)
            .add_plugins(ModulesPlugin)
            .add_systems(Update, drop.run_if(on_event::<ModuleDropped>()))
            .add_systems(Startup, grid::spawn_grid);
    }
}

fn drop (
    mut event_drop: EventReader<ModuleDropped>,
) {
    for event in event_drop.read() {
        info! ("event: ", event.0);
    }
}
