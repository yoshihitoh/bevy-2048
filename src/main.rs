use bevy::{color::palettes::css::*, prelude::*};
use bevy_2048::GamePlugin;
use bevy_prototype_lyon::prelude::*;

fn main() -> anyhow::Result<()> {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, GamePlugin))
        .run();

    Ok(())
}
