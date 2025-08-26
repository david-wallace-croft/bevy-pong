use super::collision::Collision;
use super::position::Position;
use super::shape::Shape;
use super::velocity::Velocity;
use ::bevy::math::bounding::{
  Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume,
};
use ::bevy::prelude::*;

const BALL_SIZE: f32 = 5.;

const BALL_SPEED: f32 = 5.;

#[derive(Component)]
#[require(
  Position,
  Shape = Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
  Velocity = Velocity(Vec2::new(-1., 1.)),
)]
pub struct Ball;

fn collide_with_side(
  ball: BoundingCircle,
  wall: Aabb2d,
) -> Option<Collision> {
  if !ball.intersects(&wall) {
    return None;
  }

  let closest: Vec2 = wall.closest_point(ball.center());

  let offset: Vec2 = ball.center() - closest;

  let side: Collision = if offset.x.abs() > offset.y.abs() {
    if offset.x < 0. {
      Collision::Left
    } else {
      Collision::Right
    }
  } else if offset.y > 0. {
    Collision::Top
  } else {
    Collision::Bottom
  };

  Some(side)
}

pub fn handle_collisions(
  mut ball_query: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
  without_ball_query: Query<(&Position, &Shape), Without<Ball>>,
) {
  if let Ok((mut ball_velocity, ball_position, ball_shape)) =
    ball_query.single_mut()
  {
    for (position, shape) in &without_ball_query {
      if let Some(collision) = collide_with_side(
        BoundingCircle::new(ball_position.0, ball_shape.0.x),
        Aabb2d::new(position.0, shape.0 / 2.),
      ) {
        match collision {
          Collision::Left => {
            ball_velocity.0.x *= -1.;
          },
          Collision::Right => {
            ball_velocity.0.x *= -1.;
          },
          Collision::Top => {
            ball_velocity.0.y *= -1.;
          },
          Collision::Bottom => {
            ball_velocity.0.y *= -1.;
          },
        }
      }
    }
  }
}

pub fn move_ball(ball_single: Single<(&mut Position, &Velocity), With<Ball>>) {
  let (mut position, velocity) = ball_single.into_inner();

  position.0 += velocity.0 * BALL_SPEED;
}

pub fn spawn_ball(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  println!("Spawning ball...");

  let shape: Circle = Circle::new(BALL_SIZE);

  let color: Color = Color::srgb(1., 0., 0.);

  let mesh: Handle<Mesh> = meshes.add(shape);

  let material: Handle<ColorMaterial> = materials.add(color);

  commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}
