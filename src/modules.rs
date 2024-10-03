use bevy::{
    prelude::*,
    color::palettes::css::*,
};
use bevy_vector_shapes::prelude::*;
use avian3d::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct ModulesPlugin;

#[derive(Resource)]
struct ModuleSize(f32);

impl Plugin for ModulesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(Shape2dPlugin::default())
            .add_plugins(DefaultPickingPlugins)
            .insert_resource(DebugPickingMode::Noisy)
            .insert_resource(ModuleSize(50_f32))
            .add_systems(Startup, test)
            .add_systems(Update, draw_modules);
    }
}


fn test(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::Z*10.),
        ..default()
    });
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 50.0, 5.0),
    ));
    commands.spawn(ModuleBundle::new("simple_hull".to_string(), Transform::from_xyz(100.,0.,0.)));//, Collider::cuboid(50.0, 50.0, 5.0)));
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
    shape: ModuleShape,
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
    transform: Transform,
    rigid_body: RigidBody,
    facing: Facing,
    collider: Collider,
}

impl ModuleBundle {
    pub fn new(module_name: String, transform: Transform) -> Self {
        ModuleBundle {
            module: Module::new(module_name),
            transform: transform,
            rigid_body: RigidBody::Dynamic,
            facing: Facing::Up,
            collider: Self::shape_to_collider(Self::module.shape, 50_f32),//Collider::cuboid(50.0, 50.0, 5.0),
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
        let shape = Self::name_to_shape(name.clone());
        Module {
            name: name.clone(),
            shape: shape.clone(),
            //collider: Collider::cuboid(50.0, 50.0, 5.0),//Self::shape_to_collider(shape.clone(), 50.),
        }
    }

    fn name_to_shape(module_name: String) -> ModuleShape {
        match module_name.as_str() {
            "simple_hull" => ModuleShape::Quad,
            _ => panic!(),
        }
    }
}
