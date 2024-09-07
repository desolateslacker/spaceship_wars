mod modules;

use bevy::{
    prelude::*,
    app::App,
};
use crate::modules::*;//ModulesPlugin;
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
            .add_plugins(ModulesPlugin)
            .add_systems(Startup, test);


    }
}

fn test(
    mut commands: Commands,
) {
    commands.spawn(ModuleBundle::new("simple_hull".to_string(), Transform::from_xyz(0.,0.,0.)));
}