mod modules;
mod grid;

use bevy::{
    prelude::*,
    app::App,
};
use bevy_vector_shapes::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::modules::ModulesPlugin;
use crate::modules::ModuleDropped;
use crate::grid::GridPlugin;
use crate::grid::Grid;

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
        ModuleDropped(event.target, event.pointer_location.clone())
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
    grid_query: Query<&Grid>,
    mut painter: ShapePainter,
) {
    
    
    for event in event_drop.read() {
        painter.circle(57.);
        if let Ok(grid) = grid_query.get_single() {
            painter.line((0.,0.,0.).into(), (100., 0.,0.).into());
            grid.place_module(event.0, event.1.clone(), painter);
        }
    }
}
