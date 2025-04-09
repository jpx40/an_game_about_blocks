use bevy::{math::vec2, prelude::*, window::PrimaryWindow};

use crate::{camera::MainCamera, GameState};

pub struct ScenePlugin;

impl Plugin for  ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

const HEIGHT: usize = 10 ;
const WITDH: usize = 10 ;

type Field = [[Block;WITDH];HEIGHT];

#[derive(Component)]
pub struct  Block {
    id: Entity
}

#[derive(Component)]
pub struct  MovingBlock {
    id: Entity
}

impl Block {
 fn default_color() -> Color{
    Color::WHITE
}}
#[derive(Component)]
pub struct BlockField {
    pub field: Vec<Vec<Block>>
}
const BLOCK_HEIGHT: f32 = 10. ;
const BLOCK_WITDH: f32 = 10. ;
pub fn setup(mut command:Commands, mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<StandardMaterial>>,) {

  let mut  field = BlockField {field : Vec::new()} ;
    for i in 0..HEIGHT {
        let i = i as f32;
        let mut row = Vec::new();

        for u in 0..WITDH {
        let u = u as f32;
        let id = command.spawn((
            Mesh2d(meshes.add(
                Rectangle {half_size: Vec2::new(BLOCK_HEIGHT, BLOCK_HEIGHT)}))
,MeshMaterial2d(materials.add(Block::default_color()))
Transform::from_xyz(BLOCK_WITDH * i, BLOCK_WITDH * i, 0.))
        ).id();
        row.push(Block{id});

    }
    field.field.push(row);

}
command.spawn((field));
   }

   fn spawn_new_block( mut command:Commands, q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
     q_window: Query<&Window, With<PrimaryWindow>>,
    mut block_field: Query<&mut BlockField,With<BlockField>>
) {
    if   block_field.is_empty() {
        return;
    }
           let (camera, camera_transform) = q_camera.single();
   if let Some(position) = q_window.single().cursor_position() {
       let position = camera.viewport_to_world(camera_transform, position);

       if let Ok(mut ray) = position {
           let position = ray.origin.truncate();
           let (block_field) = block_field.single_mut();
        if !block_field.field.is_empty() {
          let len =  block_field.field[0].len();
          let pos_top: &Transform = command.get_entity( block_field.field[0][0].id).get_or_insert(&Transform);
          
          let pos_bottom: &Transform = command.get_entity( block_field.field[0][block_field.field[0].len() -1].id).get_or_insert(&Transform);
          let top_y =  pos_top.translation.y;
          
          let bottom_y =  pos_top.translation.y + (block_field.field.len() as f32 * BLOCK_HEIGHT);
         for i in 0..len {
             let pos_left_x  = pos_top.translation.x + (BLOCK_WITDH * i) ;
        
             let pos_left_x  = pos_top.translation.x + (BLOCK_WITDH * (i + 1.) ) ;
             
             if  {
                 
             }
       }
   }
}
}}