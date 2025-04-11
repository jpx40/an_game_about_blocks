use bevy::{
    color::palettes::css::PLUM,
    math::{ops::asin, vec2, VectorSpace},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{camera::MainCamera, GameState};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

const HEIGHT: usize = 10;
const WITDH: usize = 10;

type Field = [[Block; WITDH]; HEIGHT];

#[derive(Component)]
pub struct Block {
    pub id: Entity,
    pub player: i32,
    pub row: usize,
    pub column: usize,
}
#[derive(Component, PartialEq)]
enum MovingBlockState {
    NotVisable,
    Moving,
    Placed,
    NotInit,
}
#[derive(Component)]
pub struct MovingBlock {
    pub row: usize,
    pub column: usize,

    pub player: i32,
    pub advancement: f32,
    pub positon: Vec2,
    pub state: MovingBlockState,
}
#[derive(Component)]
pub struct ActivePlayer {
    pub id: i32,
}
#[derive(Component)]
pub struct CurrentMovingBlock;

impl Block {
    fn new(id: Entity, row: usize, column: usize) -> Self {
        Self {
            id,
            row,
            column,
            player: 0,
        }
    }

    fn default_color() -> Color {
        Color::WHITE
    }
}
#[derive(Component)]
pub struct BlockField {
    pub field: Vec<Vec<Block>>,
}
const BLOCK_HEIGHT: f32 = 10.;
const BLOCK_WITDH: f32 = 10.;
pub fn setup(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut field = BlockField {
        field: Vec::with_capacity(HEIGHT),
    };
    for i in 0..HEIGHT {
        let i = i as f32;
        let mut row = Vec::with_capacity(HEIGHT);

        for u in 0..WITDH {
            let u = u as f32;
            let id = command
                .spawn((
                    Mesh2d(meshes.add(Rectangle {
                        half_size: Vec2::new(BLOCK_HEIGHT, BLOCK_HEIGHT),
                    })),
                    MeshMaterial2d(materials.add(Block::default_color())),
                    Transform::from_xyz(BLOCK_WITDH * i, BLOCK_WITDH * i, 0.),
                ))
                .id();
            row.push(Block::new(id, i as usize, u as usize));
        }
        field.field.push(row);
    }

    let mb = MovingBlock {
        row: 0,
        advancement: 0.,
        column: 0,
        player: 0,
        positon: Vec2::ZERO,
        state: MovingBlockState::NotInit,
    };
    command.spawn((field));
    command.spawn((mb, CurrentMovingBlock));
    let player = ActivePlayer{id: 0};
       command.spawn((player));
}

fn spawn_new_block(
    world: &mut World,
    mut command: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut block_field: Query<&mut BlockField, With<BlockField>>,
    mut player: Query<&mut ActivePlayer, With<ActivePlayer>>,
    mut moving_block: Query<&mut MovingBlock, With<CurrentMovingBlock>>,
) {
    if player.is_empty() {
        return;
    }
    let mut player = player.single_mut();
    if block_field.is_empty() {
        return;
    }
    if moving_block.is_empty() {
        return;
    }
    let (block_field) = block_field.single_mut();

    if block_field.field.is_empty() {
        return;
    }
    let mut moving_block = moving_block.single_mut();
    let (camera, camera_transform) = q_camera.single();
    if moving_block.state != MovingBlockState::Moving {
        if let Some(position) = q_window.single().cursor_position() {
            let position = camera.viewport_to_world(camera_transform, position);

            if let Ok(mut ray) = position {
                let position = ray.origin.truncate();

                let len = block_field.field[0].len();
                let pos_top = world
                    .entity(block_field.field[0][0].id)
                    .get_components::<&Transform>()
                    .unwrap();

                let pos_bottom = world
                    .entity(block_field.field[0][block_field.field[0].len() - 1].id)
                    .get_components::<&Transform>()
                    .unwrap();
                let top_y = pos_top.translation.y;

                let bottom_y = pos_bottom.translation.y + (BLOCK_HEIGHT);

                for i in 0..len {
                    let i = i as f32;
                    let pos_right_x = pos_top.translation.x + (BLOCK_WITDH * i);

                    let pos_left_x = pos_top.translation.x + (BLOCK_WITDH * (i + 1.));

                    if pos_right_x <= position.x
                        && position.x >= pos_left_x - 1.
                        && top_y <= position.y
                        && position.y >= bottom_y
                    {
                        let mb = MovingBlock {
                            row: i as usize,
                            advancement: 0.,
                            column: 0,
                            player: player.id,
                            positon: Vec2::new(pos_right_x, 0.),
                            state: MovingBlockState::NotVisable,
                        };

                        *moving_block = mb;

                        if player.id == 0 {
                            player.id = 1;
                        } else {
                            player.id = 0;
                        }
                    }
                }
            }
        }
    }
}
