use bevy::{
    prelude::*,
    color::palettes::css::*,
};
use bevy_vector_shapes::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_grid);
    }
}

#[derive(Component)]
struct Grid;

fn draw_grid(
    mut painter: ShapePainter,
    grid: Query<&Transform, With<Grid>>,
) {
    let grid = grid.get_single_mut().expect("err");
    
}

pub fn spawn_grid(
    commands: Commands,
) {
    commands.spawn((
        Grid,
        TransformBundle::default(),
    ));
}
