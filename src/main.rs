use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED:      f32   = 500.0;
pub const PLAYER_SIZE:       f32   =  64.0;
pub const ENEMY_SIZE:        f32   =  64.0;
pub const NUMBER_OF_ENEMIES: usize =     5;
pub const ENEMY_SPEED:       f32   = 100.0; 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_system(confine_player_movement)
        .add_system(confine_enemy_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .run();
}


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            sprite: Default::default(),
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            global_transform: Default::default(),
            texture: asset_server.load("sprites/PNG/Default/ball_blue_large.png"),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        },
        Player {},
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(
         Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = 32. + random::<f32>() * (window.width() - 64.);
        let random_y: f32 = 32. + random::<f32>() * (window.height() - 64.);

        commands.spawn(( 
                SpriteBundle { 
                    transform: Transform::from_xyz(random_x, random_y, 0.),
                    texture: asset_server.load("sprites/PNG/Default/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }

}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        }
        if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }
        player_transform.translation = translation;
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let window: &Window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_enemy_size;
        let x_max: f32 = window.width() - half_enemy_size;
        let y_min: f32 = 0.0 + half_enemy_size;
        let y_max: f32 = window.height() - half_enemy_size;

        let mut translation = enemy_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        }
        if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }
        enemy_transform.translation = translation;
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for(mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    player_query: Query<&Transform, &Player>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.;
    let x_min = 0. + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0. + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    //let player = player_query.get_single().unwrap();

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;
        let translation: Vec3 = transform.translation;

        if let Ok(player_transform) = player_query.get_single() {
            let distance: f32 = player_transform.translation.distance(translation);
            
            if distance < half_enemy_size * 2.{
                direction_changed = true;
            }
        }
        
        if translation.y < y_min {
            enemy.direction.y = enemy.direction.y.abs();
            direction_changed = true;
        }
        if translation.y > y_max {
            enemy.direction.y = -(enemy.direction.y.abs());
            direction_changed = true;
        }
        if translation.x < x_min {
            enemy.direction.x = enemy.direction.x.abs();
            direction_changed = true;
        }
        if translation.x > x_max {
            enemy.direction.x = -(enemy.direction.x.abs());
            direction_changed = true;
        }

        if direction_changed {
            audio.play( asset_server.load("audio/impactGlass_heavy_000.ogg"));
        }
    }
    
}
