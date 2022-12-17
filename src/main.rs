mod components;
mod systems;

use bevy::{prelude::*, render::camera::ScalingMode, time::FixedTimestep};
use components::AsteroidBundle;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Little Conflict".to_string(),
                width: 1024.0,
                height: 768.0,
                resizable: false,
                mode: WindowMode::Windowed,
                ..Default::default()
            },

            ..Default::default()
        }))
        .add_startup_system(startup)
        .add_system(systems::collide_asteroids_with_boundaries)
        .add_system(systems::collide_asteroids)
        .add_system(systems::translate_asteroids)
        // .add_system(systems::remove_asteroids)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.))
                .with_system(systems::rotate_asteroids),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));

    let asteroid_materials = (0..12)
        .map(|i| Color::hsl(i as f32 * 30.0, 1.0, 0.5))
        .chain((0..12).map(|i| Color::hsl((((i * 30) + 15) % 360) as f32, 0.5, 0.25).into()))
        .map(|base_color| {
            materials.add(StandardMaterial {
                base_color,
                metallic: 0.05,
                perceptual_roughness: 0.2,
                ..Default::default()
            })
        })
        .collect::<Vec<_>>();

    for _ in 0..100 {
        let material = asteroid_materials[rng.gen_range(0..asteroid_materials.len())].clone();
        commands.spawn(AsteroidBundle::with_rng(&mut rng, mesh.clone(), material));
    }

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        point_light: PointLight {
            intensity: 10000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal(2.0),
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
}
