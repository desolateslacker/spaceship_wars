mod grid;

use bevy::{
    prelude::*,
    color::palettes::css::*,
};
use bevy_vector_shapes::prelude::*;
use avian3d::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::pointer::Location;
use crate::modules::grid::GridPlugin;

pub struct ModulesPlugin;

#[derive(Resource)]
struct ModuleSize(f32);

impl Plugin for ModulesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                Shape2dPlugin::default(),
                DefaultPickingPlugins,
                GridPlugin,
            ))
            .insert_resource(DebugPickingMode::Noisy)
            .insert_resource(ModuleSize(50_f32))
            .insert_resource(Gravity(Vec3::NEG_Y * 10.))
            .add_event::<ModuleDropped>()
            .add_systems(Startup, test)
            .add_systems(Update, draw_modules);
    }
}

#[derive(Event)]
pub struct ModuleDropped(pub Entity, pub Location);

fn test(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::Z*10.),
        ..default()
    });
    /*
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 50.0, 5.0),
    ));
    */
    ModuleBundle::spawn_module(commands,"simple_hull".to_string(), Transform::from_xyz(100.,0.,0.));
    //commands.spawn(ModuleBundle::new("simple_hull".to_string(), Transform::from_xyz(100.,0.,0.)));//, Collider::cuboid(50.0, 50.0, 5.0)));
}

pub fn draw_modules(
    mut painter: ShapePainter,
    modules: Query<(&Transform, &Module)>,
) {
    //painter.circle(5.);
    
    modules.iter().for_each(| (tf, module) | {
        painter.reset();
        painter.set_translation(tf.translation);
        match module.name.as_str() {
            "simple_hull" => {
                painter.set_color(GREEN);
                painter.rect(Vec2::new(2. * 50., 2. * 50.));
            },
            _ => panic!(),
        }
        //painter.circle(1.);
    })
}

#[derive(Component)]
pub struct Module {
    name: String,
}

#[derive(Component)]
enum Facing {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
#[derive(Component)]
enum ModuleShape {
    Dot,
    Line,
    Angle,
    Quad,
    Hexrect,
}

#[derive(Bundle)]
pub struct ModuleBundle {
    module: Module,
    rigid_body: RigidBody,
    facing: Facing,
    shape: ModuleShape,
    collider: Collider,
}

impl ModuleBundle {
    pub fn new(module_name: String, transform: Transform) -> Self {
        ModuleBundle {
            module: Module::new(module_name.clone()),
            rigid_body: RigidBody::Dynamic,
            facing: Facing::Up,
            shape: Self::name_to_shape(module_name.clone()),
            collider: Self::shape_to_collider(Self::name_to_shape(module_name), 50_f32),
        }
    }

    pub fn spawn_module(
        mut commands: Commands,
        name: String,
        pos: Transform,
    ) {
        commands.spawn((
            Self::new(name, pos),
            TransformBundle::default(),
            PickableBundle::default(),
            On::<Pointer<DragStart>>::target_insert(RigidBody::Static),
            On::<Pointer<DragEnd>>::send_event::<ModuleDropped>(),
            //run(Snap::module_dropped),
            //target_insert(RigidBody::Dynamic),
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x;
                transform.translation.y -= drag.delta.y;
                //info! ("dd - {:?}", drag.delta);
            }),
        ));
    }

    fn name_to_shape(module_name: String) -> ModuleShape {
        match module_name.as_str() {
            "simple_hull" => ModuleShape::Quad,
            _ => panic!(),
        }
    }
    
    fn shape_to_collider(
        shape: ModuleShape, 
        module_size: f32,
    ) -> Collider {
        match shape {
            ModuleShape::Dot => Collider::cuboid(module_size, module_size, 10.),
            ModuleShape::Line => Collider::cuboid(2. * module_size, module_size, 10.),
            ModuleShape::Angle => {
                let mut collider = Vec::new();
                collider.push((Position::from_xyz(0., 0., 0.), Quat::IDENTITY, Collider::cuboid(2. * module_size, module_size, 10.)));
                collider.push((Position::from_xyz(0., module_size, 0.), Quat::IDENTITY, Collider::cuboid(module_size, module_size, 10.)));
                Collider::compound(collider)
            },
            ModuleShape::Quad => Collider::cuboid(2. * module_size, 2. * module_size, 10.),
            ModuleShape::Hexrect => Collider::cuboid(3. * module_size, 2. * module_size, 10.),
            //_ => panic!(),
        }
    }
}

impl Module {
    fn new(name: String) -> Self {
        //let shape = Self::name_to_shape(name.clone());
        Module {
            name: name,
            //shape: shape.clone(),
            //collider: Collider::cuboid(50.0, 50.0, 5.0),//Self::shape_to_collider(shape.clone(), 50.),
        }
    }
}
