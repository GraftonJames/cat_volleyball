use bevy::prelude::*;

const ARENA_WIDTH: f32 = 200.0;
const ARENA_HEIGHT: f32 = 200.0;
const PLAYER_WIDTH: f32 = 22.0;
const PLAYER_HEIGHT: f32 = 32.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cat Volleyball".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    side: Side,
}

fn initalize_player(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    cat_sprite: usize,
    side: Side,
    x: f32,
    y: f32,
) {
    commands.spawn((
        Player { side },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(cat_sprite),
            texture_atlas: atlas,
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<
    Assets<TextureAtlas>>) {
    let spritesheet = asset_server.load("textures/spritesheet.png");
    let mut sprite_atlas = TextureAtlas::new_empty(spritesheet, Vec2::new(58.0, 34.0));

    let left_cat_corner = Vec2::new(11.0,1.0);
    let right_cat_corner = Vec2::new(35.0, 1.0);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 1.0),
        ..default()
    });
    let cat_sprite = asset_server.load("texture/pixel-cat.png");
    initalize_player(
        &mut commands,
        cat_sprite.clone(),
        Side::Left,
        PLAYER_WIDTH / 2.0,
        PLAYER_HEIGHT / 2.0,
    );
    initalize_player(
        &mut commands,
        cat_sprite.clone(),
        Side::Right,
        ARENA_WIDTH - PLAYER_WIDTH / 2.0,
        PLAYER_HEIGHT / 2.0,
    );
}
