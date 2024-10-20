mod modules;

use bevy::{
    prelude::*,
    app::App,
};
use bevy_vector_shapes::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::modules::ModulesPlugin;
use crate::modules::ModuleDropped;

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
            .add_plugins(ModulesPlugin);
            //.add_systems(Update, drop.run_if(on_event::<ModuleDropped>()));
    }
}

/*
fn drop (
    mut event_drop: EventReader<ModuleDropped>,
    grid_query: Query<&Grid>,
    mut painter: &ShapePainter,
) {
    
    
    for event in event_drop.read() {
        //let mut value = grid.place_module(event.0, event.1.clone(), painter);
        *painter.circle(57.);
        if let Ok(grid) = grid_query.get_single() {
            *painter.line((0.,0.,0.).into(), (100., 0.,0.).into());
            //value;
            grid.place_module(event.0, event.1.clone(), *painter);
        }
    }
}
*/
