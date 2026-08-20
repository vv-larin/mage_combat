use bevy::prelude::*;

#[derive(Component)]
struct Mage;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
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
