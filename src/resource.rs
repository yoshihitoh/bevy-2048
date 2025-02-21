use crate::game::Board;
use bevy::prelude::*;

#[derive(Resource)]
pub struct BoardResource {
    board: Board,
}

impl BoardResource {
    pub fn board(&self) -> &Board {
        &self.board
    }
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardResource {
            board: Board::default(),
        });
    }
}
