use bevy::prelude::*;

mod ai;
mod ball;
mod camera;
mod collision;
mod constants;
mod gutter;
mod paddle;
mod player;
mod position;
mod shape;
mod velocity;

fn main() {
  let mut app: App = App::new();

  let app: &mut App = app.add_plugins(DefaultPlugins);

  let app: &mut App = app.add_systems(
    Startup,
    (
      ball::spawn_ball,
      camera::spawn_camera,
      gutter::spawn_gutters,
      paddle::spawn_paddles,
    ),
  );

  let app: &mut App = app.add_systems(
    Update,
    (
      ball::move_ball,
      paddle::handle_player_input,
      paddle::move_paddles.after(paddle::handle_player_input),
      position::project_positions.after(ball::move_ball),
      ball::handle_collisions.after(ball::move_ball),
    ),
  );

  let _app_exit: AppExit = app.run();
}
