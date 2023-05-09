//! Player

use crate::{common::*, CountdownCompleteEvent, GameAssets, GameState, GROUND_Y};
use bevy::{app::Plugin, prelude::*, sprite::collide_aabb::collide};
//use bevy_kira_audio::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

/// Scaling factor for player sprite.
const PLAYER_SCALE: f32 = 0.75; //2.75

/// Starting frame for idle animation.
const IDLE_FRAME_START: usize = 54; //30 //36

/// Gravity strength.
pub(crate) const GRAVITY: f32 = -9.8 * 250.0;

/// Initial velocity for player jumps.
const JUMP_VELOCITY: f32 = 12.0 * 100.0;

/// Velocity for horizontal player movement.
const HORIZ_VELOCITY: f32 = 5.0 * 100.0;

/// Collider alpha (used for displaying collider for debugging).
const COLLIDER_ALPHA: f32 = 0.5;

/// Starting health stat.
const MAX_HEALTH: u8 = 100;

/// Animation frame used to determine collisions a player's attack.
const ATTACK_FRAMES_LP: [usize; 2] = [21, 18]; //
const ATTACK_FRAMES_HP: [usize; 2] = [15, 14]; //
const ATTACK_FRAMES_LK: [usize; 2] = [8, 7]; //
const ATTACK_FRAMES_HK: [usize; 2] = [3, 2]; //4, 2

/// Animation frame used to determine audio for attack.
//const ATTACK_AUDIO_FRAMES: [usize; 2] = [3, 3];

/// Attack damage of player
//const ATTACK_DAMAGES: [u8; 2] = [10_u8, 8_u8]; //5, 5
const ATTACK_DAMAGESLP: [u8; 2] = [5_u8, 2_u8]; //
const ATTACK_DAMAGESHP: [u8; 2] = [10_u8, 8_u8]; //
const ATTACK_DAMAGESLK: [u8; 2] = [6_u8, 5_u8]; //
const ATTACK_DAMAGESHK: [u8; 2] = [11_u8, 9_u8]; //

lazy_static! {
    /// Frame ranges for player states (min, max).
    static ref FRAMES: [HashMap<State, (usize, usize)>; 2] = {
        let mut p1 = HashMap::new();
        p1.insert(State::Dying, (24, 28)); //6, 10 (sheet) //73, 77 (copy) //24, 28 (sprites)
        p1.insert(State::Falling, (42, 43)); //25, 26 (sheet) //91, 92 (copy) //42, 43 (sprites)
        p1.insert(State::Idling, (54, 58)); //32, 34 (sheet) //103, 107 (copy) //54, 58 (sprites)
        p1.insert(State::Jumping, (42, 43)); //25, 26 (sheet) //91, 92 (copy) //42, 43 (sprites)
        p1.insert(State::Running, (36, 41)); //18, 23 (sheet) //85, 90 (copy) //36, 41 (sprites)
        p1.insert(State::TakingHit, (30, 32)); //12, 14 (sheet) //79, 81 (copy) //30, 32 (sprites)
        p1.insert(State::LPunching, (18, 22)); //0, 4 (sheet) //67, 71 (copy) //18, 22 (sprites)
        p1.insert(State::HPunching, (12, 16)); //0, 4 (sheet) //61, 65 (copy) //12, 16 (sprites)
        p1.insert(State::LKicking, (6, 10)); //0, 4 (sheet) //54, 59 (copy) //6, 10 (sprites)
        p1.insert(State::HKicking, (0, 4)); //0, 4 (sheet) //48, 52 (copy) //0, 4 (sprites)
        //p1.insert(State::AirLPunching, (0, 5)); //18, 20
        //p1.insert(State::AirHPunching, (0, 5)); //12, 16
        //p1.insert(State::AirLKicking, (0, 5)); //6, 8
        //p1.insert(State::AirHKicking, (0, 5)); //0, 2

        let mut p2 = HashMap::new();
        p2.insert(State::Dying, (24, 28)); //6, 10 (sheet) //73, 77 (copy) //24, 28 (sprites)
        p2.insert(State::Falling, (42, 43)); //25, 26 (sheet) //91, 92 (copy) //42, 43 (sprites)
        p2.insert(State::Idling, (54, 58)); //32, 34 (sheet) //103, 107 (copy) //54, 58 (sprites)
        p2.insert(State::Jumping, (42, 43)); //25, 26 (sheet) //91, 92 (copy) //42, 43 (sprites)
        p2.insert(State::Running, (36, 41)); //18, 23 (sheet) //85, 90 (copy) //36, 41 (sprites)
        p2.insert(State::TakingHit, (30, 32)); //12, 14 (sheet) //79, 81 (copy) //30, 32 (sprites)
        p2.insert(State::LPunching, (18, 22)); //0, 4 (sheet) //67, 71 (copy) //18, 22 (sprites)
        p2.insert(State::HPunching, (12, 16)); //0, 4 (sheet) //61, 65 (copy) //12, 16 (sprites)
        p2.insert(State::LKicking, (6, 10)); //0, 4 (sheet) //54, 59 (copy) //6, 10 (sprites)
        p2.insert(State::HKicking, (0, 4)); //0, 4 (sheet) //48, 52 (copy) //0, 4 (sprites)
        //p2.insert(State::AirLPunching, (0, 5)); //18, 20
        //p2.insert(State::AirHPunching, (0, 5)); //12, 16
        //p2.insert(State::AirLKicking, (0, 5)); //6, 8
        //p2.insert(State::AirHKicking, (0, 5)); //0, 2

        [p1, p2]
    };
}

/// Handles the player mechanics.
pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthUpdateEvent>()
            // Setup the players when we enter game play.
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            // Enable all systems for game play updates.
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(collision_system)
                    .with_system(game_play_input_system.before(collision_system))
                    .with_system(movement_system.before(collision_system))
                    .with_system(animation_system)
                    .with_system(game_over_system),
            )
            // Enabling animation and movement system will ensure movement/animations can
            // complete on Game Over. Since input system is not enabled it will not allow
            // game play anymore.
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(animation_system)
                    .with_system(movement_system),
            )
            // Cleanup resources on leaving game over state.
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup));
    }
}

/// Player entities.
#[derive(Resource)]
struct EntityData {
    entities: Vec<Entity>,
}

/// Represents the player.
#[derive(Component, Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Player {
    One,
    Two,
}

impl Player {
    pub(crate) fn index(&self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }
    pub(crate) fn opponent(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

/// Represents player states.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum State {
    Dying,
    Falling,
    Idling,
    Jumping,
    Running,
    TakingHit,
    LPunching,
    HPunching,
    LKicking,
    HKicking,
}

impl Default for State {
    fn default() -> Self {
        Self::Idling
    }
}

/// Represents player's current state.
#[derive(Component, Default, Deref, DerefMut)]
struct CurrentState(State);

impl CurrentState {
    fn set_state(&mut self, state: State) {
        self.0 = state;
    }
    fn set_from_previous(&mut self, state: &PreviousState) {
        self.0 = state.0;
    }
}

/// Represents player's previous state.
#[derive(Component, Default, Deref, DerefMut)]
struct PreviousState(State);

impl PreviousState {
    fn set_state(&mut self, state: State) {
        self.0 = state;
    }
    fn set_from_current(&mut self, state: &CurrentState) {
        self.0 = state.0;
    }
}

/// Represents player velocity.
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

/// Used to adjust sprite's y location based on vertical padding in sprite sheet.
#[derive(Component)]
struct GroundY(f32);

/// Represents player action keys.
#[derive(Component)]
struct Keys {
    left: KeyCode,
    right: KeyCode,
    jump: KeyCode,
    lpunch: KeyCode,
    hpunch: KeyCode,
    lkick: KeyCode,
    hkick: KeyCode,
}

/// Represents the bounding box for testing attack collisions.
#[derive(Component)]
struct ColliderBox;

/// Represents the attack box for testing with collider_boxes.
#[derive(Component)]
struct AttackBox;

#[derive(Component)]
struct HPAttackBox;

#[derive(Component)]
struct LKAttackBox;

#[derive(Component)]
struct HKAttackBox;

/// Represents the current sprite index (used for determining collision).
#[derive(Component, Deref, DerefMut)]
struct CurrentFrame(usize);

/// Represents the health.
#[derive(Component, Deref, DerefMut)]
pub(crate) struct Health(pub(crate) u8);

/// Used to communicate changes to the player's health with other systems.
pub(crate) struct HealthUpdateEvent {
    pub(crate) player: Player,
    pub(crate) health: u8,
}

impl HealthUpdateEvent {
    pub(crate) fn new(player: Player, health: u8) -> Self {
        HealthUpdateEvent { player, health }
    }
}

/// Setup the players.
fn setup(mut commands: Commands, assets: Res<GameAssets>) { //audio: Res<Audio>
    //audio.play(assets.in_game_audio.clone()).looped();

    let mut entities: Vec<Entity> = Vec::new();

    // Player 1: Adjust player feet (Height=200, y-feet=122 => y-center=100 => y-offset=22).
    let mut pos = Vec3::new(-300.0, GROUND_Y, PLAYER_Z);
    pos += Vec3::new(0.0, 28.0 * PLAYER_SCALE, 0.01);
    entities.push(spawn_player(
        &mut commands,
        &assets,
        Player::One,
        pos,
        Keys {
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::W,
            lpunch: KeyCode::R,
            hpunch: KeyCode::T,
            lkick: KeyCode::F,
            hkick: KeyCode::G,
        },
        Vec3::new(-10.0, 0.0, PLAYER_Z + 0.02),
        Vec3::new(100.0, 250.0, 1.0) * PLAYER_SCALE,
        Color::rgba(0.0, 0.0, 1.0, COLLIDER_ALPHA),
        //lp hitbox
        Vec3::new(35.0, 50.0, PLAYER_Z + 0.03), //125.0, 35.0 | 145.0, 56.0
        Vec3::new(90.0, 45.0, 1.0) * PLAYER_SCALE, //75.0, 25.0
        Color::rgba(1.0, 0.0, 0.0, COLLIDER_ALPHA),
        // hp hitbox
        Vec3::new(35.0, 50.0, PLAYER_Z + 0.03),
        Vec3::new(90.0, 45.0, 1.0) * PLAYER_SCALE,
        //lk hitbox
        Vec3::new(35.0, -50.0, PLAYER_Z + 0.03),
        Vec3::new(90.0, 45.0, 1.0) * PLAYER_SCALE,
        //hk hitbox
        Vec3::new(60.0, 0.0, PLAYER_Z + 0.03),
        Vec3::new(125.0, 225.0, 1.0) * PLAYER_SCALE,
        // dummy color used to make individual hitboxes invisible
        Color::rgba(0.0, 0.0, 0.0, 0.0),
    ));

    // Player 2: Adjust player feet. (Height=200, y-feet=128 => y-center=100 => y-offset=28).
    pos = Vec3::new(300.0, GROUND_Y, PLAYER_Z);
    pos += Vec3::new(0.0, 28.0 * PLAYER_SCALE, 0.01);
    entities.push(spawn_player(
        &mut commands,
        &assets,
        Player::Two,
        pos,
        Keys {
            left: KeyCode::Left,
            right: KeyCode::Right,
            jump: KeyCode::Up,
            lpunch: KeyCode::L,
            hpunch: KeyCode::Semicolon,
            lkick: KeyCode::Comma,
            hkick: KeyCode::Period,
        },
        Vec3::new(10.0, 0.0, PLAYER_Z + 0.02),
        Vec3::new(100.0, 250.0, 1.0) * PLAYER_SCALE,
        Color::rgba(0.0, 0.0, 1.0, COLLIDER_ALPHA),
        //lp hitbox
        Vec3::new(-35.0, 50.0, PLAYER_Z + 0.03),
        Vec3::new(90.0, 45.0, 1.0) * PLAYER_SCALE,
        Color::rgba(1.0, 0.0, 0.0, COLLIDER_ALPHA),
        // hp hitbox
        Vec3::new(-35.0, 50.0, PLAYER_Z + 0.03),
        Vec3::new(90.0, 45.0, 1.0) * PLAYER_SCALE,
        //lk hitbox
        Vec3::new(-35.0, -50.0, PLAYER_Z + 0.03),
        Vec3::new(90.0, 45.0, 1.0) * PLAYER_SCALE,
        //hk hitbox
        Vec3::new(-60.0, 0.0, PLAYER_Z + 0.03),
        Vec3::new(125.0, 225.0, 1.0) * PLAYER_SCALE,
        // dummy color used to make individual hitboxes invisible
        Color::rgba(0.0, 0.0, 0.0, 0.0),
    ));

    commands.insert_resource(EntityData { entities });
}

/// Spawn players.
fn spawn_player(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    player: Player,
    player_pos: Vec3,
    keys: Keys,
    collider_box_pos: Vec3,
    collider_box_size: Vec3,
    collider_box_color: Color,
    attack_box_pos: Vec3,
    attack_box_size: Vec3,
    attack_box_color: Color,
    hp_attack_box_pos: Vec3,
    hp_attack_box_size: Vec3,
    lk_attack_box_pos: Vec3,
    lk_attack_box_size: Vec3,
    hk_attack_box_pos: Vec3,
    hk_attack_box_size: Vec3,
    debug_invis: Color,
) -> Entity {
    let player_atlas_handle = match player {
        Player::One => assets.player_one_texture_atlas.clone(),
        Player::Two => assets.player_two_texture_atlas.clone(),
    };

    let player_tex_scale = match player {
        Player::One => Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0), // Right facing
        Player::Two => Vec3::new(-PLAYER_SCALE, PLAYER_SCALE, 1.0), // Flip X-axis for left facing
    };

    commands
        .spawn(player)
        .insert(Health(MAX_HEALTH))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(player_pos.y))
        .insert(CurrentFrame(IDLE_FRAME_START))
        .insert(SpatialBundle {
            visibility: Visibility { is_visible: true },
            transform: Transform {
                translation: player_pos,
                ..default()
            },
            ..default()
        })
        .insert(keys)
        .insert(AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .with_children(|player| {
            player.spawn(SpriteSheetBundle {
                texture_atlas: player_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: IDLE_FRAME_START, // Idling frame start. Avoids starting at Attacking frame.
                    ..default()
                },
                transform: Transform {
                    scale: player_tex_scale,
                    ..default()
                },
                ..default()
            });

            player
                .spawn(ColliderBox)
                .insert(GroundY(collider_box_pos.y))
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: collider_box_color,
                        ..default()
                    },
                    transform: Transform {
                        translation: collider_box_pos,
                        scale: collider_box_size,
                        ..default()
                    },
                    ..default()
                });

            player
                .spawn(AttackBox)
                .insert(GroundY(attack_box_pos.y))
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: attack_box_color,
                        ..default()
                    },
                    transform: Transform {
                        translation: attack_box_pos,
                        scale: attack_box_size,
                        ..default()
                    },
                    ..default()
                });

            player
            .spawn(HPAttackBox)
            .insert(GroundY(hp_attack_box_pos.y))
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: attack_box_color,
                    ..default()
                },
                transform: Transform {
                    translation: hp_attack_box_pos,
                    scale: hp_attack_box_size,
                    ..default()
                },
                ..default()
            });

            player
            .spawn(LKAttackBox)
            .insert(GroundY(lk_attack_box_pos.y))
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: attack_box_color,
                    ..default()
                },
                transform: Transform {
                    translation: lk_attack_box_pos,
                    scale: lk_attack_box_size,
                    ..default()
                },
                ..default()
            });

            player
            .spawn(HKAttackBox)
            .insert(GroundY(hk_attack_box_pos.y))
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: attack_box_color,
                    ..default()
                },
                transform: Transform {
                    translation: hk_attack_box_pos,
                    scale: hk_attack_box_size,
                    ..default()
                },
                ..default()
            });
        })
        .id()
}

/// Handle play input.
fn game_play_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut CurrentState,
            &mut PreviousState,
            &Keys,
            &Transform,
            &GroundY,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    for (mut current_state, mut previous_state, keys, transform, ground_y, mut velocity) in
        player_query.iter_mut()
    {
        // Don't do anything if player is dead.
        match current_state.0 {
            State::Dying => continue,
            _ => (),
        }

        // Move left as long as left key is pressed.
        if keyboard_input.pressed(keys.left) {
            velocity.x = -HORIZ_VELOCITY;
        } else if keyboard_input.just_released(keys.left) {
            velocity.x = 0.0;
        }

        // Move right as long as right key is pressed.
        if keyboard_input.pressed(keys.right) {
            velocity.x = HORIZ_VELOCITY;
        } else if keyboard_input.just_released(keys.right) {
            velocity.x = 0.0;
        }

        // Jump gives an initial upward velocity which will be adjusted based on GRAVITY.
        if keyboard_input.pressed(keys.jump) {
            if transform.translation.y == ground_y.0 {
                velocity.y = JUMP_VELOCITY;
            }
        }

        if keyboard_input.pressed(keys.lpunch) {
            // If player is either attacking already or taking a hit don't allow an attack.
            match current_state.0 {
                State::LPunching | State::HPunching | State::LKicking | State::HKicking | State::TakingHit => (),
                _ => {
                    previous_state.set_from_current(&current_state);
                    current_state.set_state(State::LPunching);
                }
            }
        }

        if keyboard_input.pressed(keys.hpunch) {
            // If player is either attacking already or taking a hit don't allow an attack.
            match current_state.0 {
                State::LPunching | State::HPunching | State::LKicking | State::HKicking | State::TakingHit => (),
                _ => {
                    previous_state.set_from_current(&current_state);
                    current_state.set_state(State::HPunching);
                }
            }
        }

        if keyboard_input.pressed(keys.lkick) {
            // If player is either attacking already or taking a hit don't allow an attack.
            match current_state.0 {
                State::LPunching | State::HPunching | State::LKicking | State::HKicking | State::TakingHit => (),
                _ => {
                    previous_state.set_from_current(&current_state);
                    current_state.set_state(State::LKicking);
                }
            }
        }

        if keyboard_input.pressed(keys.hkick) {
            // If player is either attacking already or taking a hit don't allow an attack.
            match current_state.0 {
                State::LPunching | State::HPunching | State::LKicking | State::HKicking | State::TakingHit => (),
                _ => {
                    previous_state.set_from_current(&current_state);
                    current_state.set_state(State::HKicking);
                }
            }
        }
    }
}

/// Handle player movement based on velocity.
fn movement_system(
    time: Res<Time>,
    mut player_query: Query<(
        &Player,
        &mut CurrentState,
        &PreviousState,
        &mut Transform,
        &GroundY,
        &mut Velocity,
        &CurrentFrame,
        &Health,
    )>,
    app_state: Res<bevy::prelude::State<GameState>>,
) {
    let mut new_velocities = [Vec2::default(); 2];
    let mut move_x = [false; 2];
    let delta_time = time.delta_seconds();

    for (
        player,
        mut current_state,
        previous_state,
        mut transform,
        ground_y,
        mut velocity,
        current_frame,
        health,
    ) in &mut player_query
    {
        // Handle horizontal movement.
        let new_x = transform.translation.x + velocity.x * delta_time;
        if new_x > crate::scene::SCENE_MIN_X && new_x < crate::scene::SCENE_MAX_X {
            transform.translation.x = new_x;
            move_x[player.index()] = true;
        } else {
            move_x[player.index()] = false;
        }

        // Handle vertical movement.
        transform.translation.y += velocity.y * delta_time;
        if transform.translation.y > ground_y.0 {
            // Player is in the air keep decreasing velocity.
            velocity.y += GRAVITY * delta_time;
        } else if transform.translation.y <= ground_y.0 {
            // Player has hit the ground. Reset velocity and position.
            transform.translation.y = ground_y.0;
            velocity.y = 0.0;
        }

        // Check if player is dying.
        if health.0 == 0 {
            current_state.set_state(State::Dying);
        }

        // Check if game over.
        match app_state.current() {
            GameState::GameOver => {
                // Once player is on ground and not dead, move to idle state so player doesn't
                // continue running or jumping.
                if transform.translation.y <= ground_y.0 {
                    if !matches!(current_state.0, State::Dying) {
                        current_state.0 = State::Idling;
                    }
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                    continue;
                }
            }
            _ => (),
        }

        match current_state.0 {
            State::Dying => {
                // Don't do anything. Game over.
            }
            State::LPunching => {
                // Let player finish attacking.
                let max_frame = FRAMES[player.index()].get(&State::LPunching).unwrap().1;
                if current_frame.0 == max_frame {
                    current_state.set_from_previous(previous_state);
                }
            }
            State::HPunching => {
                // Let player finish attacking.
                let max_frame = FRAMES[player.index()].get(&State::HPunching).unwrap().1;
                if current_frame.0 == max_frame {
                    current_state.set_from_previous(previous_state);
                }
            }
            State::LKicking => {
                // Let player finish attacking.
                let max_frame = FRAMES[player.index()].get(&State::LKicking).unwrap().1;
                if current_frame.0 == max_frame {
                    current_state.set_from_previous(previous_state);
                }
            }
            State::HKicking => {
                // Let player finish attacking.
                let max_frame = FRAMES[player.index()].get(&State::HKicking).unwrap().1;
                if current_frame.0 == max_frame {
                    current_state.set_from_previous(previous_state);
                }
            }
            State::TakingHit => {
                // Let player finish taking hit.
                let max_frame = FRAMES[player.index()].get(&State::TakingHit).unwrap().1;
                if current_frame.0 == max_frame {
                    match previous_state.0 {
                        State::LPunching => {
                            // Don't resume attacking state after taking a hit.
                            // Determine state based on position/velocity.
                            if transform.translation.y > ground_y.0 {
                                if velocity.y > 0.0 {
                                    current_state.0 = State::Jumping;
                                } else {
                                    current_state.0 = State::Falling;
                                }
                            } else if velocity.x != 0.0 {
                                current_state.0 = State::Running;
                            } else {
                                current_state.0 = State::Idling;
                            }
                        }
                        State::HPunching => {
                            // Don't resume attacking state after taking a hit.
                            // Determine state based on position/velocity.
                            if transform.translation.y > ground_y.0 {
                                if velocity.y > 0.0 {
                                    current_state.0 = State::Jumping;
                                } else {
                                    current_state.0 = State::Falling;
                                }
                            } else if velocity.x != 0.0 {
                                current_state.0 = State::Running;
                            } else {
                                current_state.0 = State::Idling;
                            }
                        }
                        State::LKicking => {
                            // Don't resume attacking state after taking a hit.
                            // Determine state based on position/velocity.
                            if transform.translation.y > ground_y.0 {
                                if velocity.y > 0.0 {
                                    current_state.0 = State::Jumping;
                                } else {
                                    current_state.0 = State::Falling;
                                }
                            } else if velocity.x != 0.0 {
                                current_state.0 = State::Running;
                            } else {
                                current_state.0 = State::Idling;
                            }
                        }
                        State::HKicking => {
                            // Don't resume attacking state after taking a hit.
                            // Determine state based on position/velocity.
                            if transform.translation.y > ground_y.0 {
                                if velocity.y > 0.0 {
                                    current_state.0 = State::Jumping;
                                } else {
                                    current_state.0 = State::Falling;
                                }
                            } else if velocity.x != 0.0 {
                                current_state.0 = State::Running;
                            } else {
                                current_state.0 = State::Idling;
                            }
                        }
                        _ => {
                            // Resume previous state.
                            current_state.set_from_previous(previous_state);
                        }
                    }
                }
            }
            _ => {
                // Determine state based on position/velocity.
                if transform.translation.y > ground_y.0 {
                    if velocity.y > 0.0 {
                        current_state.0 = State::Jumping;
                    } else {
                        current_state.0 = State::Falling;
                    }
                } else if velocity.x != 0.0 {
                    current_state.0 = State::Running;
                } else {
                    current_state.0 = State::Idling;
                }
            }
        }

        // Store positions for collider_boxes.
        new_velocities[player.index()] = Vec2::new(velocity.x, velocity.y);
    }
}

/// Handle collision detection.
fn collision_system(
    mut player_query: Query<(
        &Player,
        &mut CurrentState,
        &mut PreviousState,
        &CurrentFrame,
        &mut Health,
    )>,
    collider_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<ColliderBox>>,
    attack_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<AttackBox>>,
    hp_attack_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<HPAttackBox>>,
    lk_attack_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<LKAttackBox>>,
    hk_attack_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<HKAttackBox>>,
    mut health_update_events: EventWriter<HealthUpdateEvent>,
) {
    // Since we need to check one player's collider with the opponent's attack_box we need to
    // load this information before running the collision detection.
    let mut players = [(State::default(), State::default(), 0_usize); 2];
    for (player, current_state, previous_state, current_frame, _health) in &player_query {
        players[player.index()] = (current_state.0, previous_state.0, current_frame.0);
    }

    let mut collider_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &collider_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        collider_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    let mut attack_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &attack_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        attack_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    let mut hp_attack_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &hp_attack_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        hp_attack_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    let mut lk_attack_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &lk_attack_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        lk_attack_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    let mut hk_attack_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &hk_attack_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        hk_attack_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    // Check collision detection.
    for (player, mut current_state, mut previous_state, _current_frame, mut health) in
        &mut player_query
    {
        match current_state.0 {
            State::TakingHit | State::Dying => continue,
            _ => (),
        }

        let opponent = player.opponent().index();
        let (collider_box_pos, collider_box_size) = collider_boxes[player.index()];
        let opponent_attack_frame_lp = ATTACK_FRAMES_LP[opponent];
        let opponent_attack_frame_hp = ATTACK_FRAMES_HP[opponent];
        let opponent_attack_frame_lk = ATTACK_FRAMES_LK[opponent];
        let opponent_attack_frame_hk = ATTACK_FRAMES_HK[opponent];
        let (opponent_current_state, _opponent_previous_state, opponent_current_frame) =
            players[opponent];
        let (opponent_attack_box_pos, opponent_attack_box_size) = attack_boxes[opponent];
        let (opponent_hp_attack_box_pos, opponent_hp_attack_box_size) = hp_attack_boxes[opponent];
        let (opponent_lk_attack_box_pos, opponent_lk_attack_box_size) = lk_attack_boxes[opponent];
        let (opponent_hk_attack_box_pos, opponent_hk_attack_box_size) = hk_attack_boxes[opponent];
        //let opponent_attack_damage = ATTACK_DAMAGES[opponent];
        let opponent_attack_damage_lp = ATTACK_DAMAGESLP[opponent]; //
        let opponent_attack_damage_hp = ATTACK_DAMAGESHP[opponent]; //
        let opponent_attack_damage_lk = ATTACK_DAMAGESLK[opponent]; //
        let opponent_attack_damage_hk = ATTACK_DAMAGESHK[opponent]; //

        match opponent_current_state {
            State::LPunching => {
                if opponent_current_frame == opponent_attack_frame_lp { //LP
                    if collide(
                        opponent_attack_box_pos,
                        opponent_attack_box_size,
                        collider_box_pos,
                        collider_box_size,
                    )
                    .is_some()
                    {
                        // Switch state to TakingHit.
                        previous_state.set_state(current_state.0);
                        current_state.set_state(State::TakingHit);

                        // Just in case damage is not a nice divisior of MAX_HEALTH.
                        if let Some(h) = health.0.checked_sub(opponent_attack_damage_lp) { //LP//
                            health.0 = h;
                        } else {
                            health.0 = 0;
                        }
                        health_update_events.send(HealthUpdateEvent::new(*player, health.0));
                    }
                }
            }
            State::HPunching => { //HP
                if opponent_current_frame == opponent_attack_frame_hp {
                    if collide(
                        opponent_hp_attack_box_pos,
                        opponent_hp_attack_box_size,
                        collider_box_pos,
                        collider_box_size,
                    )
                    .is_some()
                    {
                        // Switch state to TakingHit.
                        previous_state.set_state(current_state.0);
                        current_state.set_state(State::TakingHit);

                        // Just in case damage is not a nice divisior of MAX_HEALTH.
                        if let Some(h) = health.0.checked_sub(opponent_attack_damage_hp) { //HP//
                            health.0 = h;
                        } else {
                            health.0 = 0;
                        }
                        health_update_events.send(HealthUpdateEvent::new(*player, health.0));
                    }
                }
            }
            State::LKicking => { //LK
                if opponent_current_frame == opponent_attack_frame_lk {
                    if collide(
                        opponent_lk_attack_box_pos,
                        opponent_lk_attack_box_size,
                        collider_box_pos,
                        collider_box_size,
                    )
                    .is_some()
                    {
                        // Switch state to TakingHit.
                        previous_state.set_state(current_state.0);
                        current_state.set_state(State::TakingHit);

                        // Just in case damage is not a nice divisior of MAX_HEALTH.
                        if let Some(h) = health.0.checked_sub(opponent_attack_damage_lk) { //LK//
                            health.0 = h;
                        } else {
                            health.0 = 0;
                        }
                        health_update_events.send(HealthUpdateEvent::new(*player, health.0));
                    }
                }
            }
            State::HKicking => {
                if opponent_current_frame == opponent_attack_frame_hk { //HK
                    if collide(
                        opponent_hk_attack_box_pos,
                        opponent_hk_attack_box_size,
                        collider_box_pos,
                        collider_box_size,
                    )
                    .is_some()
                    {
                        // Switch state to TakingHit.
                        previous_state.set_state(current_state.0);
                        current_state.set_state(State::TakingHit);

                        // Just in case damage is not a nice divisior of MAX_HEALTH.
                        if let Some(h) = health.0.checked_sub(opponent_attack_damage_hk) { //HK//
                            health.0 = h;
                        } else {
                            health.0 = 0;
                        }
                        health_update_events.send(HealthUpdateEvent::new(*player, health.0));
                    }
                }
            }
            _ => (),
        }
    }
}

/// Animate the player sprite.
fn animation_system(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Player,
            &CurrentState,
            &mut AnimationTimer,
            &mut CurrentFrame,
        ),
        With<Player>,
    >,
    mut sprite_query: Query<(&Parent, &mut TextureAtlasSprite)>,
    assets: Res<GameAssets>,
    //audio: Res<Audio>,
) {
    for (parent, mut sprite) in &mut sprite_query {
        let (player, current_state, mut animation_timer, mut current_frame) =
            player_query.get_mut(parent.get()).unwrap();

        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            let (frame, _looped) = next_frame(player, current_state.0, sprite.index);
            sprite.index = frame;
            current_frame.0 = frame;

/* 
            if sprite.index == ATTACK_AUDIO_FRAMES[player.index()] {
                match player {
                    Player::One => {
                        audio.play(assets.player_one_attack_audio.clone());
                    }
                    Player::Two => {
                        audio.play(assets.player_two_attack_audio.clone());
                    }
                }
            }
*/
        }
    }
}

/// Gets next animation frame for player.
fn next_frame(player: &Player, state: State, current: usize) -> (usize, bool) {
    let (start, end) = FRAMES[player.index()].get(&state).unwrap();
    let (frame, looped) = next_player_sprite_frame(current, *start, *end);

    match state {
        State::Dying => {
            // Don't loop dying animation.
            if looped {
                (*end, false)
            } else {
                (frame, looped)
            }
        }
        _ => (frame, looped),
    }
}

/// Returns the next frame for player sprite.
fn next_player_sprite_frame(mut current: usize, start: usize, end: usize) -> (usize, bool) {
    if current < start || current > end {
        // Out of bounds for current player state. Reset to start.
        (start, false)
    } else {
        current = current + 1;
        if current > end {
            (start, true)
        } else {
            (current, false)
        }
    }
}

/// Checks if game is over.
fn game_over_system(
    mut countdown_complete_events: EventReader<CountdownCompleteEvent>,
    mut health_update_events: EventReader<HealthUpdateEvent>,
    mut app_state: ResMut<bevy::prelude::State<GameState>>,
) {
    let mut game_over = false;

    // Check if countdown is complete.
    if !countdown_complete_events.is_empty() {
        for _event in countdown_complete_events.iter() {
            game_over = true;
            break;
        }
    }

    if !game_over {
        // Check if one player has 0 health.
        if !health_update_events.is_empty() {
            for event in health_update_events.iter() {
                if event.health == 0 {
                    game_over = true;
                    break;
                }
            }
        }
    }

    if game_over {
        // Transition game state.
        app_state.set(GameState::GameOver).unwrap();
    }
}

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
