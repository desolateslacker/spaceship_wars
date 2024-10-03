use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_grid);
    }
}

fn spawn_grid(
    commands: Commands,
) {
    commands.spawn(
        
    );
}
