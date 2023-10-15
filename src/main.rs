use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite, movement, follow))
        .run();
}

#[derive(Component)]
struct Character;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn follow(
    player: Query<&Transform, With<Character>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Character>)>,
) {
    let player_transform = player.single();
    let mut camera_transform = camera.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn movement(
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Character>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let (mut transform, mut sprite) = query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += 100. * time.delta_seconds();
        transform.translation.z += 1.;
    }

    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= 100. * time.delta_seconds();
        transform.translation.z -= 1.;
    }

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= 100. * time.delta_seconds();
        sprite.flip_x = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 100. * time.delta_seconds();
        sprite.flip_x = false;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        NodeBundle {
            style: Style {
                height: Val::Percent(30.),
                width: Val::Percent(30.),
                ..default()
            },
            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        }
    );

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Character,
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    });
}