use crate::components::{Stats, Velocity};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub fn stats(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<Stats>>) {
    let mut text = query.single_mut();
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(raw) = fps.value() {
            text.sections[1].value = format!("{raw:.2}");
        }
        if let Some(sma) = fps.average() {
            text.sections[3].value = format!("{sma:.2}");
        }
        if let Some(ema) = fps.smoothed() {
            text.sections[5].value = format!("{ema:.2}");
        }
    }
}

pub fn collide_asteroids(mut query: Query<(&mut Transform, &mut Velocity)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(mut transform_a, mut velocity_a), (mut transform_b, mut velocity_b)]) =
        iter.fetch_next()
    {
        let distance_vec = transform_a.translation - transform_b.translation;
        let distance = distance_vec.length();
        let scale_a = transform_a.scale.x;
        let scale_b = transform_b.scale.x;
        let scale = scale_a + scale_b;
        let diff = scale - distance;
        if diff > 0.0 {
            let norm = distance_vec.normalize();
            transform_a.translation += norm * diff / 2.0;
            transform_b.translation -= norm * diff / 2.0;
            std::mem::swap(&mut velocity_a.0, &mut velocity_b.0);
            let mass_a = scale_a * scale_a;
            let mass_b = scale_b * scale_b;
            velocity_a.0 *= mass_b / mass_a;
            velocity_b.0 *= mass_a / mass_b;
        }
    }
}

pub fn collide_asteroids_with_boundaries(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        let scale = transform.scale.x;
        if transform.translation.x.abs() > (1.0 - scale) {
            transform.translation.x = transform.translation.x.clamp(-1.0 + scale, 1.0 - scale);
            velocity.0.x = -velocity.0.x;
        }
        if transform.translation.y.abs() > (1.0 - scale) {
            transform.translation.y = transform.translation.y.clamp(-1.0 + scale, 1.0 - scale);
            velocity.0.y = -velocity.0.y;
        }
    }
}

pub fn translate_asteroids(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}

pub fn rotate_asteroids(mut query: Query<&mut Transform, With<Velocity>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(1. / 60.));
    }
}
