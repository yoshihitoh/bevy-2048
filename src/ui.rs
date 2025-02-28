use bevy::prelude::*;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::prelude::{BorderRadii, RectangleOrigin};
use bevy_prototype_lyon::shapes;
use std::collections::HashMap;

use crate::AppState;
use crate::game::{Tile, Value};
use crate::resource::BoardResource;

#[derive(Debug, Event)]
pub enum UiEvent {
    TileSpawned { slot_no: usize, tile: Tile }, // FIXME: スロットからタイルを取得するようにする
    TileMoved { slot_from: usize, slot_to: usize },
    TileMerged { slot_from: usize },
    GameOver,
}

mod styles;

#[derive(Resource)]
struct StartupTimer(Timer);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(styles::window::background_color()))
            .insert_resource(StartupTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_event::<UiEvent>()
            .add_systems(OnEnter(AppState::InGame), init)
            .add_systems(
                Update,
                (inject_debugging_events, update).run_if(in_state(AppState::InGame)),
            );
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

    let cell_size = styles::slot::SLOT_SIZE;
    let cell_rect = shapes::Rectangle {
        extents: Vec2::splat(cell_size),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(styles::slot::BORDER_RADII)),
    };

    dbg!((board_size, cell_size));

    for r in 0..board.grid.rows {
        for c in 0..board.grid.cols {
            let x = (-board_size / 2.0) + (spacing / 2.0) + (c as f32 * spacing);
            let y = (board_size / 2.0) - (spacing / 2.0) - (r as f32 * spacing);

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

fn inject_debugging_events(
    time: Res<Time>,
    mut timer: ResMut<StartupTimer>,
    mut ui_events: EventWriter<UiEvent>,
) {
    if timer.0.finished() {
        return;
    }

    if timer.0.tick(time.delta()).finished() {
        let debugging_events = vec![
            UiEvent::TileSpawned {
                slot_no: 0,
                tile: Tile {
                    value: Value::from(2),
                },
            },
            UiEvent::TileSpawned {
                slot_no: 5,
                tile: Tile {
                    value: Value::from(2),
                },
            },
            UiEvent::TileSpawned {
                slot_no: 15,
                tile: Tile {
                    value: Value::from(8),
                },
            },
        ];

        for event in debugging_events {
            ui_events.send(event);
        }
    }
}

fn update(
    _board_resource: Res<BoardResource>,
    mut commands: Commands,
    mut slot_query: Query<(Entity, &Transform, &UiSlot)>,
    mut ui_events: EventReader<UiEvent>,
) {
    // TODO: マージイベント・タイル移動イベントを受け付けて更新するように変更する
    let ui_events = ui_events.read().collect::<Vec<_>>();
    if ui_events.is_empty() {
        return;
    }

    dbg!("start update");

    let slots = slot_query
        .iter()
        .map(|(entity, transform, slot)| (slot.slot_no, (entity, transform, slot)))
        .collect::<HashMap<_, _>>();

    let tile_size = styles::tile::TILE_SIZE;
    let tile_rect = shapes::Rectangle {
        extents: Vec2::splat(tile_size),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(styles::tile::BORDER_RADII)),
    };

    for event in ui_events {
        match event {
            UiEvent::TileSpawned { slot_no, tile } => {
                let &(slot_entity, slot_transform, ui_slot) = slots.get(slot_no).unwrap();
                commands.entity(slot_entity).with_children(|parent| {
                    parent
                        .spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&tile_rect),
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            },
                            Fill::color(styles::tile::background_color_for(tile.value)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text2d::new(format!("{}", tile.value.as_u32())),
                                TextFont {
                                    font_size: 60.0,
                                    ..default()
                                },
                                TextColor(styles::tile::foreground_color_for(tile.value)),
                                Transform::from_xyz(0.0, 0.0, 1.0),
                            ));
                        });
                });
            }
            _ => {}
        }
    }

    dbg!("finish update");
}
