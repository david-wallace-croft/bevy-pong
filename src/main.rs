use bevy::prelude::*;

mod ai;
mod ai_score;
mod ball;
mod camera;
mod collision;
mod constants;
mod gutter;
mod paddle;
mod player;
mod player_score;
mod position;
mod score;
mod scoreboard;
mod scored;
mod scorer;
mod shape;
mod velocity;

fn main() {
  let mut app: App = App::new();

  let app: &mut App = app.add_plugins(DefaultPlugins);

  let app: &mut App = app.init_resource::<score::Score>();

  let app: &mut App = app.add_event::<scored::Scored>();

  let app: &mut App = app.add_systems(
    Startup,
    (
      ball::spawn_ball,
      camera::spawn_camera,
      gutter::spawn_gutters,
      paddle::spawn_paddles,
      scoreboard::spawn_scoreboard,
    ),
  );

  let app: &mut App = app.add_systems(
    Update,
    (
      ball::move_ball,
      player::handle_player_input,
      ball::detect_scoring,
      ai::move_ai,
      ball::reset_ball.after(ball::detect_scoring),
      score::update_score.after(ball::detect_scoring),
      scoreboard::update_scoreboard.after(score::update_score),
      paddle::move_paddles.after(player::handle_player_input),
      position::project_positions.after(ball::move_ball),
      ball::handle_collisions.after(ball::move_ball),
    ),
  );

  let _app_exit: AppExit = app.run();
}
