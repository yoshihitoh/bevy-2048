use bevy::prelude::*;

pub mod game;
pub mod resource;
mod ui;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, States, Default)]
pub enum AppState {
    /// 再起動
    Restart,

    /// ローディング
    #[default]
    Loading,

    /// ゲーム
    InGame,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins((resource::ResourcePlugin, ui::UiPlugin))
            .add_systems(OnEnter(AppState::Restart), on_restart)
            .add_systems(OnEnter(AppState::Loading), on_loading);
    }
}

fn on_restart(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Loading);
}

fn on_loading(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}
