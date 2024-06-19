use std::sync::Arc;

use bevy::{
    app::{App, Startup},
    asset::AssetMetaCheck,
    ecs::system::Commands,
    prelude::*,
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vello::{
    debug::DebugVisualizations,
    integrations::{VectorFile, VelloAsset},
    vello::Scene,
    vello_svg::{
        render_tree,
        usvg::{fontdb::Database, Options, Tree},
    },
    VelloAssetBundle, VelloPlugin,
};
use perma_planner::CartoClient;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VelloPlugin, WorldInspectorPlugin::new()))
        .add_systems(Startup, load_svg)
        .insert_resource(AssetMetaCheck::Never)
        .run();
}

fn load_svg(mut commands: Commands, mut assets: ResMut<Assets<VelloAsset>>) {
    let svg = CartoClient::get_commune_svg("78646").expect("unable to load svg");

    commands.spawn(Camera2dBundle::default());

    let svg = Tree::from_str(&svg, &Options::default(), &Database::default()).expect("Prout");

    let mut scene = Scene::new();

    render_tree(&mut scene, &svg);

    // Yes, it's this simple.
    let handle = assets.add(VelloAsset {
        file: VectorFile::Svg(Arc::new(scene)),
        local_transform_center: Transform::default(),
        width: 200.,
        height: 200.,
        alpha: 1.,
    });

    commands.spawn(VelloAssetBundle {
        vector: handle,
        debug_visualizations: DebugVisualizations::Visible,
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    });
}
