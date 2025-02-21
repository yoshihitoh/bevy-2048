use bevy::prelude::*;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{BorderRadii, RectangleOrigin};
use bevy_prototype_lyon::shapes;

use crate::AppState;
use crate::resource::BoardResource;

mod styles;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(styles::window::background_color()))
            .add_systems(OnEnter(AppState::InGame), init)
            .add_systems(Update, (update).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct UiBoard;

#[derive(Component, Debug)]
struct UiSlot {
    slot_no: usize,
}

#[derive(Component)]
struct UiTile;

fn init(
    board_resource: Res<BoardResource>,
    mut commands: Commands,
    _asset_server: ResMut<AssetServer>,
) {
    commands.spawn((Camera2d, Msaa::Sample4));

    let board = board_resource.board();
    let spacing = styles::slot::SPACING;
    let board_size = spacing * board.grid.rows as f32;
    let board_rect = shapes::Rectangle {
        extents: Vec2::splat(board_size),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(styles::board::BORDER_RADII)),
    };
    let board_entity = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&board_rect),
                ..default()
            },
            Stroke::new(styles::board::border_color(), styles::board::BORDER_SIZE),
            Fill::color(styles::board::background_color()),
        ))
        .insert(UiBoard)
        .id();

    let cell_size = styles::cell::CELL_SIZE;
    let cell_rect = shapes::Rectangle {
        extents: Vec2::splat(cell_size),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(styles::cell::BORDER_RADII)),
    };

    dbg!((board_size, cell_size));

    for r in 0..board.grid.rows {
        for c in 0..board.grid.cols {
            let x = (-board_size / 2.0) + (spacing / 2.0) + (c as f32 * spacing);
            let y = (-board_size / 2.0) + (spacing / 2.0) + (r as f32 * spacing);

            commands.entity(board_entity).with_children(|parent| {
                let slot_no = r * board.grid.rows + c;
                parent
                    .spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&cell_rect),
                            transform: Transform::from_xyz(x, y, 1.0),
                            ..default()
                        },
                        Fill::color(styles::slot::background_color()),
                    ))
                    .insert(UiSlot { slot_no });
            });
        }
    }
}

fn update(_board_resource: Res<BoardResource>, mut slot_query: Query<(&Transform, &UiSlot)>) {
    // TODO: マージイベント・タイル移動イベントを受け付けて更新するように変更する

    dbg!("start update");
    for (transform, slot) in slot_query.iter_mut() {
        dbg!(slot.slot_no);
    }
    dbg!("finish update");
}
