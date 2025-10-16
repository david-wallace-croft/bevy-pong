use ::bevy::app::PluginGroupBuilder;
use ::bevy::prelude::*;
use ::bevy::window::PresentMode;

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

  let canvas: Option<String> = Some("#bevy-pong-canvas".into());

  let resize_constraints = WindowResizeConstraints {
    min_width: 500.,
    min_height: 300.,
    max_width: 500.,
    max_height: 300.,
  };

  let window: Window = Window {
    canvas,
    fit_canvas_to_parent: true,
    name: Some("bevy.app".into()),
    present_mode: PresentMode::AutoVsync,
    prevent_default_event_handling: false,
    resizable: false,
    resize_constraints,
    resolution: (500, 300).into(),
    title: "I am a window!".into(),
    ..default()
  };

  let primary_window: Option<Window> = Some(window);

  let window_plugin: WindowPlugin = WindowPlugin {
    primary_window,
    ..default()
  };

  let default_plugins_plugin_group_builder: PluginGroupBuilder =
    DefaultPlugins.set(window_plugin);

  let app: &mut App = app.add_plugins(default_plugins_plugin_group_builder);

  let app: &mut App = app.init_resource::<score::Score>();

  let app: &mut App = app.add_message::<scored::Scored>();

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
