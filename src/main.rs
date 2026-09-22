use bevy::prelude::*;

/// Scale of Boid mesh
const BOID_SCALE: f32 = 10.0;

/// Color of boid when cannot see others
const BOID_COLOR_ALONE: Color = Color::linear_rgba(0.0, 0.0, 1.0, 0.5);

/// Color of boid when can see others
const BOID_COLOR_HAPPY: Color = Color::linear_rgba(0.0, 1.0, 0.0, 0.5);

/// Color of boid when about to collide
const BOID_COLOR_AVERT: Color = Color::linear_rgba(1.0, 0.0, 0.0, 0.5);

#[derive(Component, Debug, Default, Deref)]
struct Position(Vec2);

#[derive(Component, Debug, Default, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct AvoidRadius(u32);

#[derive(Component, Default)]
struct SightRadius(f32);

#[derive(Component)]
struct FaceVelocity;

#[derive(Component)]
#[require(Position, Velocity, AvoidRadius, SightRadius, Mesh2d)]
struct Boid;


/// Invisibile point in the centroid of the flock, used for cohesion steering
/// force calculations
#[derive(Component)]
struct FlockCentroid;

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut vel) in &mut query {
        if transform.translation.x > 500.0 {
            vel.x = -1.0 * f32::abs(vel.x);
        } else if transform.translation.x < -500.0 {
            vel.x = f32::abs(vel.x);
        }
        if transform.translation.y > 300.0 {
            vel.y = -1.0 * f32::abs(vel.y);
        } else if transform.translation.y < -300.0 {
            vel.y = f32::abs(vel.y);
        }

        transform.translation.x += vel.x * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();
    }
}

fn face_velocity(mut query: Query<(&mut Transform, &Velocity), With<FaceVelocity>>) {
    for (mut transform, vel) in &mut query {
        let rot = Quat::from_rotation_arc_2d(Vec2::Y, vel.normalize());
        println!("{:?}", rot);
        transform.rotation = rot;
    }
}

fn add_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    for n in 0..10 {
        let triangle = Triangle2d::new(
            Vec2::Y * BOID_SCALE,
            Vec2::new(-BOID_SCALE, -BOID_SCALE),
            Vec2::new(BOID_SCALE, -BOID_SCALE)
        );
        commands.spawn((
            Mesh2d(meshes.add(triangle)),
            MeshMaterial2d(materials.add(BOID_COLOR_ALONE)),

            Boid,
            Position(Vec2 { x: 0.0, y: (n as f32) * 0.0 }),
            Velocity(Vec2 { x: f32::sin(n as f32 * 36.0) * 64.0, y: f32::sin(n as f32 * 12.0) * 32.0 }),
            AvoidRadius(1),
            SightRadius(2.0),

            FaceVelocity,
        ));
    }
}


fn check_for_sightings(
    mut commands: Commands,
    mut observer_query: Query<(Entity, &Transform, &SightRadius, &mut Handle<ColorMaterial>), With<Boid>>,
    mut observed_query: Query<&Transform, With<Boid>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (ent, observer_pos, observer_radius, cmat_handle) in observer_query {  // TODO Definitely wrong
        for observed_pos in observed_query {
            let x1 = observer_pos.translation.x;
            let y1 = observer_pos.translation.y;
            let x2 = observed_pos.translation.x;
            let y2 = observed_pos.translation.y;

            let dist = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
            let SightRadius(r) = observer_radius;
            if dist <= r {
                let cmat = materials.get_mut(cmat_handle).unwwrap();
                commands.entity(ent).insert(materials.add(ColorMaterial::from(BOID_COLOR_HAPPY)));
            }
        }
    }
}

fn step_boids(query: Query<(&Position, &Velocity), With<Boid>>) {
    for (pos, vel) in query {
    }
}

pub struct BoidPlugin;
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_boids);
        app.add_systems(Update, step_boids);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BoidPlugin)
        .add_systems(FixedUpdate, (
            apply_velocity,
            face_velocity,
            check_for_sightings
        ).chain())
        .run()
    ;
}
