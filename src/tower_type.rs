use bevy::prelude::*;
use crate::{Bullet, BulletBundle, GameAssets, Movement, Tower, TowerBundle};
use strum_macros::{EnumIter, Display};
use bevy_inspector_egui::Inspectable;

#[derive(EnumIter, Inspectable, Component, Display, Clone, Copy, Debug, PartialEq)]
pub enum TowerType {
  Nature,
  Fire,
  Ice,
  Dark,
  Mage,
  Archmage
}

impl TowerType {
  pub fn get_tower(&self, assets: &GameAssets, position: Vec3) -> TowerBundle {
    match self {
      TowerType::Nature => TowerBundle {
          tower_type: TowerType::Nature,
          tower: Tower::new(
            Vec3::new(20., 0., 0.),
            1,
            Timer::from_seconds(1., TimerMode::Repeating),
            10,
            100
          ),
          sprite: SpriteBundle {
            texture: assets.wizard_nature.clone(),
            transform: Transform::from_translation(position),
            ..default()
          },
          name: Name::new("NatureTower")
      },
      TowerType::Fire => TowerBundle {
        tower_type: TowerType::Fire,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_fire.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("FireTower")
      },
      TowerType::Ice => TowerBundle {
        tower_type: TowerType::Ice,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_ice.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("IceTower")
      },
      TowerType::Dark => TowerBundle {
        tower_type: TowerType::Dark,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_dark.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("DarkTower")
      },
      TowerType::Mage => TowerBundle {
        tower_type: TowerType::Mage,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_mage.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("MageTower")
      },
      TowerType::Archmage => TowerBundle {
        tower_type: TowerType::Archmage,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_archmage.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("ArchmageTower")
      }
    }
  }
  
  pub fn get_bullet(&self, damage: i32, assets: &GameAssets, position: Transform) -> BulletBundle {
    match self {
      TowerType::Nature => BulletBundle {
          bullet: Bullet {
            damage,
            lifetime: Timer::from_seconds(2., TimerMode::Once),
          },
          movement: Movement {
            direction: Vec3::new(0.00000001,0.,0.),
            speed: 1500.
          },
          sprite: SpriteBundle {
            texture: assets.wizard_nature_bullet.clone(),
            transform: position,
            ..default()
          },
          name: Name::new("Bullet")
      },
      TowerType::Fire => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        sprite: SpriteBundle {
          texture: assets.wizard_fire_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Ice => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        sprite: SpriteBundle {
          texture: assets.wizard_ice_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Dark => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        sprite: SpriteBundle {
          texture: assets.wizard_dark_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Mage => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        sprite: SpriteBundle {
          texture: assets.wizard_mage_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Archmage => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        sprite: SpriteBundle {
          texture: assets.wizard_archmage_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      }
    }
  }
  
  pub fn path(&self, assets: &GameAssets) -> Handle<Image> {
    match self {
      TowerType::Nature => assets.wizard_nature_button.clone(),
      TowerType::Fire => assets.wizard_fire_button.clone(),
      TowerType::Ice => assets.wizard_ice_button.clone(),
      TowerType::Dark => assets.wizard_dark_button.clone(),
      TowerType::Mage => assets.wizard_mage_button.clone(),
      TowerType::Archmage => assets.wizard_archmage_button.clone()
    }
  }
}