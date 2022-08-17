use crate::prelude::*;

// COMPONENTS

// Taking suggestions from https://gafferongames.com/post/integration_basics/
#[derive(Component)]
pub struct SemiImplicitEulerPhysics {
    pub vel: Vec2,
    pub accel: Vec2,
}

impl SemiImplicitEulerPhysics {
    pub fn zero() -> Self {
        Self {
            vel: Vec2::ZERO,
            accel: Vec2::ZERO,
        }
    }

    pub fn stop(&mut self) {
        self.vel = Vec2::ZERO;
        self.accel = Vec2::ZERO;
    }
}

#[derive(Component)]
pub struct GravityAffected(f32);

impl GravityAffected {
    pub fn default() -> Self {
        Self(20.0)
    }
}

// SYSTEMS

pub fn update_and_apply_physics(
    time: Res<Time>,
    mut grav_query: Query<(
        &GravityAffected,
        &mut SemiImplicitEulerPhysics,
        &mut Transform,
    )>,
    mut nograv_query: Query<
        (&mut SemiImplicitEulerPhysics, &mut Transform),
        Without<GravityAffected>,
    >,
) {
    let dt = time.delta_seconds();

    // with gravity
    for (grav, mut phys, mut trans) in grav_query.iter_mut() {
        phys.vel.x += phys.accel.x * dt;
        phys.vel.y += (phys.accel.y - grav.0) * dt;
        trans.translation.x += phys.vel.x * dt;
        trans.translation.y += phys.vel.y * dt;
    }

    // without gravity
    for (mut phys, mut trans) in nograv_query.iter_mut() {
        phys.vel.x += phys.accel.x * dt;
        phys.vel.y += phys.accel.y * dt;
        trans.translation.x += phys.vel.x * dt;
        trans.translation.y += phys.vel.y * dt;
    }
}
