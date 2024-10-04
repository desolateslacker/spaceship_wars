use bevy::{
    prelude::*,
    color::palettes::css::*,
};
use bevy_vector_shapes::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, Grid::draw_grid);
    }
}

#[derive(Component)]
struct Grid{
    width: i32,
    height: i32,
    storage: Vec<i32>,
};

impl Grid {
    pub fn new(
        width: i32, 
        height: i32,
    ) -> Grid {
        Grid {
            width,
            height,
            storage: Vec::new(),
        }
    }

    pub fn draw_grid(
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
}
