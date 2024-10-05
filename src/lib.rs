mod modules;
mod grid;

use bevy::{
    prelude::*,
    app::App,
};
use crate::modules::ModulesPlugin;
use crate::grid::GridPlugin;

pub struct GamePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    _Hangar,
    _Battle,
}

#[derive(Event)]
struct DropModule(Entity, Option<Vec3>);

impl From<ListenerInput<Pointer<DragEnd>>> for DropModule {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        DropModule(event.target, event.hit.position)
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

fn drop () {}
