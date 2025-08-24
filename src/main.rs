use bevy::prelude::*;

mod ball;
mod camera;
mod paddle;
pub(crate) mod position;

fn main() {
  let mut app: App = App::new();

  let app: &mut App = app.add_plugins(DefaultPlugins);

  let app: &mut App = app.add_systems(
    Startup,
    (
      ball::spawn_ball,
      camera::spawn_camera,
      paddle::spawn_paddles,
    ),
  );

  let app: &mut App = app.add_systems(
    Update,
    (
      ball::move_ball,
      position::project_positions.after(ball::move_ball),
    ),
  );

  let _app_exit: AppExit = app.run();
}
