use bevy::{input::keyboard::KeyboardInput, prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: 500.0,
            width: 500.0,
            resizable: false,
            ..WindowDescriptor::default()
        })
        .insert_resource(ClearColor(Color::PINK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(input)
        .run();
}

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(ImageBundle {
        material: materials.add(ass.load("ball.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
            ..Style::default()
        },
        ..ImageBundle::default()
    });
}

struct Overlay;

fn input(
    mut commands: Commands,
    mut er: EventReader<KeyboardInput>,
    query: Query<Entity, With<Overlay>>,
) {
    use bevy::input::ElementState::*;

    for ev in er.iter() {
        if ev.state == Pressed {
            if let Ok(entity) = query.get_single() {
                commands.entity(entity).despawn_recursive();
            } else {
                commands
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..Style::default()
                        },
                        ..NodeBundle::default()
                    })
                    .insert(Overlay);
            }
        }
    }
}
