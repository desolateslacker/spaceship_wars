use bevy::{
    prelude::*,
    asset::AssetMetaCheck,
};
use spaceship_wars::GamePlugin;
use bevy_mod_picking::prelude::*;
fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
        .add_plugins(
            DefaultPlugins
                .set(low_latency_window_plugin())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Spaceship Wars".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
        )
        .add_plugins(GamePlugin)
        .run();
}
