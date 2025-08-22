use bevy::prelude::*;

fn main() {
  let mut app: App = App::new();

  let app: &mut App = app.add_plugins(DefaultPlugins);

  let app: &mut App = app.add_systems(Startup, hello_world);

  let _app_exit: AppExit = app.run();
}

fn hello_world() {
  println!("Hello, World!");
}
