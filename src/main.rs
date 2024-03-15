use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, initialize_game)
        .add_systems(Update, update_game)
        .run();
}

fn initialize_game() {
    
}

fn update_game() {

}