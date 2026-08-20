use bevy::prelude::*;

const MAGE_SPEED: f32 = 100.0;

#[derive(Component)]
struct Mage;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, move_mage);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mage,
        Sprite::from_color(Color::WHITE, Vec2::new(20.0, 20.0)),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}

fn move_mage(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mage>>,
) {
    let mut transform = match query.single_mut() {
        Ok(transform) => transform,
        Err(error) => {
            panic!("There should be exactly one mage: {error}");
        }
    };

    let mut move_dir = Vec2::ZERO;

    if key.pressed(KeyCode::KeyW) {
        move_dir.y += 1.0;
    }
    if key.pressed(KeyCode::KeyS) {
        move_dir.y -= 1.0;
    }
    if key.pressed(KeyCode::KeyD) {
        move_dir.x += 1.0;
    }
    if key.pressed(KeyCode::KeyA) {
        move_dir.x -= 1.0;
    }

    transform.translation +=
        MAGE_SPEED * time.delta_secs() * move_dir.normalize_or_zero().extend(0.0);
}
