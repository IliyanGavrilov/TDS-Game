use bevy::prelude::*;
use crate::{GameAssets, MainCamera, spawn_tower, TowerType};
use strum::IntoEnumIterator;

pub struct TowerButtonPlugin;

impl Plugin for TowerButtonPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(generate_ui)
       .add_system(tower_button_interaction)
       .add_system(place_tower);
  }
}

// Marker component to despawn buttons in UI
#[derive(Component)]
pub struct TowerUIRoot;

// Convert cursor position from window/screen position to world position
fn window_to_world_pos(
  window: &Window,
  cursor_pos: Vec2,
  camera: &Camera,
  camera_transform: &GlobalTransform
) -> Vec3 {
  // get the size of the window
  let window_size = Vec2::new(window.width() as f32, window.height() as f32);
  
  // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
  // Normalized device coordinates
  let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
  
  // matrix for undoing the projection and camera transform
  let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
  
  // use it to convert ndc to world-space coordinates
  let mut world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
  
  world_pos.z = 0.;
  
  return world_pos;
}

#[derive(Component)]
pub struct SpriteFollower;

fn place_tower(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Transform, &TowerType), With<SpriteFollower>>,
  assets: Res<GameAssets>,
  mouse: Res<Input<MouseButton>>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  for (entity, mut transform, tower_type) in query.iter_mut() {
    if let Some(position) = window.cursor_position() {
      transform.translation =
        window_to_world_pos(window, position, camera, camera_transform);
    }
    // Spawn the tower if user clicks with mouse button in a valid tower placement zone!!!
    if mouse.just_pressed(MouseButton::Left) {
      if let Some(screen_pos) = window.cursor_position() {
        commands.entity(entity).despawn_recursive();
        spawn_tower(&mut commands,
                    *tower_type,
                    &assets,
                    window_to_world_pos(window,
                                        screen_pos,
                                        camera,
                                        camera_transform));
      }
    } else if mouse.just_pressed(MouseButton::Right) || window.cursor_position().is_none() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn tower_button_interaction(
  mut commands: Commands,
  assets: Res<GameAssets>,
  interaction: Query<(&Interaction, &TowerType), (Changed<Interaction>, With<Button>)>,
  mut images: Query<&mut UiImage>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  for (interaction, tower_type) in &interaction {
    match interaction {
      Interaction::Clicked => {
        info!("Spawning: {tower_type} wizard");
        // Change button UI!!!
        for mut image in images.iter_mut() {
          image.0 = assets.get_button_pressed_asset(*tower_type);
        }
        
        // Spawn component that alerts the place_tower() system that a button has been pressed
        // and it starts moving a sprite with the cursor until the tower is placed
        if let Some(position) = window.cursor_position() {
          commands.spawn(SpriteBundle {
            texture: assets.get_tower_asset(*tower_type),
            transform: Transform::from_translation(
              window_to_world_pos(window, position, camera, camera_transform)),
            ..default()
          })
            .insert(SpriteFollower)
            .insert(*tower_type);
        }
      }
      Interaction::Hovered => {
        // Change button UI!!!
        for mut image in images.iter_mut() {
          image.0 = assets.get_button_hovered_asset(*tower_type);
        }
      }
      Interaction::None => { // Change button UI
        // Change button UI!!!
        // for mut image in images.iter_mut() {
        //   image.0 = assets.get_button_asset(*tower_type);
        // }
      }
    }
  }
}

// Creating a UI menu on the whole screen with buttons
fn generate_ui(mut commands: Commands, assets: Res<GameAssets>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    })
    .insert(TowerUIRoot) // Marker component
    .with_children(|commands| { // Make the buttons children of the menu
      for i in TowerType::iter() {
        commands
          .spawn(ButtonBundle {
            style: Style {
              size: Size::new(Val::Px(80.), Val::Px(80.)),
              align_self: AlignSelf::FlexEnd, // Bottom of screen
              margin: UiRect::all(Val::Percent(2.0)),
              ..default()
            },
            image: assets.get_button_asset(i).into(),
            ..default()
          })
          .insert(i);
      }
    });
}