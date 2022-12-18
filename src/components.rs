use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Default)]
pub struct Stats;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Bundle)]
pub struct AsteroidBundle {
    pbr: PbrBundle,
    velocity: Velocity,
}

impl AsteroidBundle {
    pub fn with_rng(
        rng: &mut (impl Rng + ?Sized),
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        let pbr = PbrBundle {
            mesh,
            transform: Transform {
                translation: Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0),
                rotation: random_quat(rng),
                scale: Vec3::splat(rng.gen_range(0.02..=0.05)),
                ..Default::default()
            },
            material,
            ..Default::default()
        };
        let velocity = Velocity(Vec2::new(
            rng.gen_range(-0.5..=0.5),
            rng.gen_range(-0.5..=0.5),
        ));
        Self { pbr, velocity }
    }
}

fn random_quat(rng: &mut (impl Rng + ?Sized)) -> Quat {
    use std::f32::consts::TAU;
    let u = rng.gen_range(0.0..=1.0);
    let v = rng.gen_range(0.0..=1.0);
    let w = rng.gen_range(0.0..=1.0);
    let i = 1f32 - u;
    Quat::from_xyzw(
        i.sqrt() * (TAU * v).sin(),
        i.sqrt() * (TAU * v).cos(),
        u.sqrt() * (TAU * w).sin(),
        u.sqrt() * (TAU * w).cos(),
    )
}
