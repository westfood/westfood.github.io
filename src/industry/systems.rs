use bevy::{prelude::*, transform::commands};
use super::components::*;

pub fn upprade_industry(){}
pub fn damage_industry(){}
pub fn destroy_industry(){}
pub fn change_industry(){}
pub fn create_industry(mut commands: Commands, industry: MineTypes, yeald: u32){
    commands.spawn((Mine::new(industry), IronYeald { value: yeald }));
}