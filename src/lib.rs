mod modules;

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

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins(GridPlugin)
            .add_plugins(ModulesPlugin)
            .add_systems(Startup, GridPlugin::spawn_grid);
    }
}
