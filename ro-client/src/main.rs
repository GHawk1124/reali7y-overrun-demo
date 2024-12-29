use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Replace this with your own `helpers` mod path
mod helpers;

// -------------------
//  Game States
// -------------------
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    InGame,
}

// -------------------
//  Asset Collection
// -------------------
#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "finite_embedded.tmx")]
    map: Handle<helpers::tiled::TiledMap>,
}

// -------------------
//   Bevy App
// -------------------
fn main() {
    App::new()
        // 1. Core Bevy plugins with configuration
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tiled Map Editor Example".into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                // Disable meta check for certain environments
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
        // 2. Tilemap plugin
        .add_plugins(TilemapPlugin)
        // 3. Custom Tiled plugin
        .add_plugins(helpers::tiled::TiledMapPlugin)
        // 4. Asset loading state
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::InGame)
                .load_collection::<GameAssets>(),
        )
        // 5. Systems
        .add_systems(OnEnter(GameState::InGame), spawn_game)
        .add_systems(Update, helpers::camera::movement)
        .run();
}

// -------------------
//   Spawn Systems
// -------------------
fn spawn_game(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn camera
    commands.spawn(Camera2d::default());

    // Spawn Tiled map
    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: helpers::tiled::TiledMapHandle(game_assets.map.clone()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
