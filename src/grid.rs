use bevy::{
    prelude::*,
    color::palettes::css::*,
};
use bevy_vector_shapes::prelude::*;
use bevy_mod_picking::pointer::Location;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, Grid::draw_grid);
    }
}

#[derive(Component)]
pub struct Grid{
    width: i32,
    height: i32,
    storage: Vec<i32>,
}

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
        grid: Query<(&Transform, &Grid)>,
    ) {
        let (tf, grid) = grid.get_single().expect("err");
        painter.reset();
        painter.translate(Vec3::NEG_Y * 25_f32 * grid.height as f32);
        painter.translate(Vec3::NEG_X * 25_f32 * grid.width as f32);
        for row in 0..=grid.height {
            painter.line(Vec3::ZERO, Vec3::X * 50_f32 * grid.width as f32);
            painter.translate(Vec3::Y * 50_f32);
        }

        painter.reset();
        painter.translate(Vec3::NEG_Y * 25_f32 * grid.height as f32);
        painter.translate(Vec3::NEG_X * 25_f32 * grid.width as f32);
        for column in 0..=grid.width {
            painter.line(Vec3::ZERO, Vec3::Y * 50_f32 * grid.height as f32);
            painter.translate(Vec3::X * 50_f32);
        }
    }

    pub fn place_module(
        &self, 
        module: Entity, 
        location: Location
    ) {
        let mut commands = Commands::new();
    }
}

pub fn spawn_grid(
    mut commands: Commands,
) {
    commands.spawn((
        Grid::new(9, 6),
        TransformBundle::default(),
    ));
}
