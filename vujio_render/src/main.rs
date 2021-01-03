use bevy::prelude::*;

fn main() {
    let mut app = App::build();
    app.add_resource(Msaa { samples: 4 })
        .init_resource::<State>()
        .add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
        .add_system(mouse_tracking_system.system())
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup.system()).run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

#[derive(Default)]
struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

struct MouseLoc(Vec2);

fn mouse_tracking_system(
    mut mouse_pos: ResMut<MouseLoc>,
    mut state: ResMut<State>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        mouse_pos.0 = event.position;
        println!("{}, {}", event.position.x, event.position.y);
    }
}
