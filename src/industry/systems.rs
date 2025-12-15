use std::collections::HashMap;

use super::components::*;
use bevy::{prelude::*, transform::commands};

pub fn report_yeald(query: Query<(&Yield)>) {
    let mut total_yields: HashMap<YieldType, u32> = HashMap::new();
    for yield_hash in query.iter() {
        for (&yield_type, &amount) in yield_hash.yields.iter() {
            *total_yields.entry(yield_type).or_insert(0) += amount;
        }
    }
    for (yield_type, total) in total_yields.iter() {
        println!("Production of {:?} {}", yield_type, total);
    }
}
pub fn upgrade_industry() {}
pub fn damage_industry() {}
pub fn destroy_industry() {}
pub fn change_industry() {}
pub fn create_industry(commands: &mut Commands, industry: &MineTypes, amount: u32) {
    match industry {
        MineTypes::Iron => {
            let mut production = Yield::new();
            production.add_yield(YieldType::Iron, amount);
            commands.spawn((Name::new(industry.name()), Mine::new(industry), production));
        }
        MineTypes::Copper => {
            let mut production = Yield::new();
            production.add_yield(YieldType::Copper, amount);
            commands.spawn((Name::new(industry.name()), Mine::new(industry), production));
        }
        MineTypes::Tin => {
            let mut production = Yield::new();
            production.add_yield(YieldType::Tin, amount);
            commands.spawn((Name::new(industry.name()), Mine::new(industry), production));
        }
    }
}
