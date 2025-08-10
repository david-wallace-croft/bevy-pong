use bevy::prelude::*;

fn main() {
  let mut app: App = App::new();

  let app: &mut App = app.add_plugins(DefaultPlugins);

  let _app_exit: AppExit = app.run();
}
