use bevy::prelude::*;
use bevy_hanabi::*;

pub struct Particles;

impl Plugin for Particles {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}

impl Particles {
    fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(0.0, 1.0, 1.0, 1.0));
        gradient.add_key(0.2, Vec4::splat(0.));

        let spawner = Spawner::once(30.0.into(), false);
        let effect = effects.add(
            EffectAsset {
                name: "Impact".into(),
                capacity: 32768,
                spawner,
                ..Default::default()
            }
            .init(PositionSphereModifier {
                radius: 1.0,
                speed: 100.0.into(),
                dimension: ShapeDimension::Surface,
                ..Default::default()
            })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(5.0)),
            })
            .render(ColorOverLifetimeModifier { gradient }),
        );

        commands
            .spawn_bundle(ParticleEffectBundle::new(effect).with_spawner(spawner))
            .insert(Name::new("particle effect"));
    }
}
