mod components;
mod systems;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::camera::ScalingMode,
    time::FixedTimestep,
};
use components::{AsteroidBundle, Stats};
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
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(startup)
        .add_startup_system(setup_stats)
        .add_system(systems::collide_asteroids_with_boundaries)
        .add_system(systems::collide_asteroids)
        .add_system(systems::translate_asteroids)
        // .add_system(systems::remove_asteroids)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.))
                .with_system(systems::rotate_asteroids),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(systems::stats),
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

fn setup_stats(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let id = db
        .query(&fontdb::Query {
            families: &[fontdb::Family::SansSerif],
            style: fontdb::Style::Normal,
            ..Default::default()
        })
        .unwrap();
    let (source, _) = db.face_source(id).unwrap();
    let handle = match source {
        fontdb::Source::File(path) => asset_server.load(path),
        _ => panic!("Unsupported font source"),
    };

    let section = |color, value: &str| {
        TextSection::new(
            value,
            TextStyle {
                font: handle.clone(),
                font_size: 20.0,
                color,
            },
        )
    };

    commands.spawn((
        Stats::default(),
        TextBundle::from_sections([
            section(Color::LIME_GREEN, "FPS (raw): "),
            section(Color::WHITE, ""),
            section(Color::LIME_GREEN, "\nFPS (SMA): "),
            section(Color::WHITE, ""),
            section(Color::LIME_GREEN, "\nFPS (EMA): "),
            section(Color::WHITE, ""),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(16.),
                top: Val::Px(16.),
                ..Default::default()
            },
            ..Default::default()
        }),
    ));
}
