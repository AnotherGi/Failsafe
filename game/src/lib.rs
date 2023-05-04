//! Figher

mod common;
mod countdown_timer;
mod game_over_menu;
mod health;
mod main_menu;
mod menu;
mod player;
mod scene;
mod utils;

use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use common::*;
use countdown_timer::*;
use game_over_menu::*;
use health::*;
use main_menu::*;
use menu::*;
use player::*;
use scene::*;

// Create the app.
pub fn run() {
    App::new()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::MainMenu)
                .with_collection::<GameAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Approaching Storms".to_string(),
                        //width: WINDOW_WIDTH,
                        //height: WINDOW_HEIGHT,
                        fit_canvas_to_parent: true,
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(AudioPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(CountdownTimerPlugin)
        .add_plugin(GameOverPlugin)
        .run();
}

/// Game assets
#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "fonts/m6x11.ttf")]
    pub(crate) font: Handle<Font>,

    #[asset(path = "images/background_composite.png")]
    pub(crate) background_image: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 118.0, tile_size_y = 128.0, columns = 15, rows = 1))]
    #[asset(path = "images/shop_anim.png")]
    pub(crate) shop_texture_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 384.0, tile_size_y = 224.0, columns = 6, rows = 10))] //2304.0, 1568.0 //6, 7
    #[asset(path = "images/Jerry_Crude_Sprites_p1.png")]
    pub(crate) player_one_texture_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 384.0, tile_size_y = 224.0, columns = 6, rows = 10))] //2304.0, 1344.0 //6, 7
    #[asset(path = "images/Jerry_Crude_Sprites_p2.png")]
    pub(crate) player_two_texture_atlas: Handle<TextureAtlas>,

    #[asset(path = "images/return_key.png")]
    pub(crate) return_key_image: Handle<Image>,

    #[asset(path = "images/escape_key.png")]
    pub(crate) escape_key_image: Handle<Image>,

    #[asset(path = "images/Title_Screen_Reference_Crude.png")]
    pub(crate) title_screen_image: Handle<Image>,

    #[asset(path = "audio/sword sound.wav")]
    pub(crate) player_one_attack_audio: Handle<AudioSource>,

    #[asset(path = "audio/melee sound.wav")]
    pub(crate) player_two_attack_audio: Handle<AudioSource>,

    #[asset(path = "audio/Adventure Theme Intro.wav")]
    pub(crate) main_menu_audio: Handle<AudioSource>,

    #[asset(path = "audio/Boss Battle 6 Metal V1.wav")]
    pub(crate) in_game_audio: Handle<AudioSource>,
}

/// Game states.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    AssetLoading,
    //MatchMaking
    MainMenu,
    InGame,
    GameOver,
}
