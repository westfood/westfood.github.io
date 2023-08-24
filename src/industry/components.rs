use bevy::prelude::*;
// use crate::gods::components::*;

#[derive(Component, Debug)]
pub struct IronYeald {
    pub value: u32,
}

#[derive(Component, Debug)]
pub struct CopperYeald {
    pub value: u32,
}

#[derive(Component, Debug)]
pub struct TinYeald {
    pub value: u32,
}

#[derive(Debug)]
pub enum MineTypes {
    Iron,
    Tin,
    Copper,
}

impl MineTypes {
    fn is_valid(&self) -> bool {
        match self {
            MineTypes::Iron | MineTypes::Tin | MineTypes::Copper => true,
            _ => false,
        }
    }
    pub fn name(&self) -> String {
        match self {
            MineTypes::Iron => String::from("Iron Mine"),
            MineTypes::Copper => String::from("Copper Mine"),
            MineTypes::Tin => String::from("Tin Mine"),
        }
    }
    pub fn kind(&self) -> String {
        match self {
            MineTypes::Iron => String::from("Iron"),
            MineTypes::Copper => String::from("Copper"),
            MineTypes::Tin => String::from("Tin"),
        }
    }
}

#[derive(Component, Debug)]
pub struct Mine {
    pub name: String,
    pub kind: String,
    asset_id: String,
    pub owner: String,
    pub yeald: u32,
    pub storage: u32,
}


impl Mine {
    pub fn new(mine_type: MineTypes, yeald: u32, god: String) -> Mine {
        if mine_type.is_valid() {
            info!("Spawning {} for {}", mine_type.name(), god);
            {
                Mine {
                    name: mine_type.name(),
                    kind: mine_type.kind(),
                    asset_id: format!("hexagon-pack/PNG/Tiles/Medieval/medieval_mine.png"),
                    owner: god,
                    yeald: yeald,
                    storage: 0
                }
            }
        } else {
            panic!("Wrong Mine Type")
        }
    }
    pub fn add_yeald(&self) {
        match self {
            _ => todo!(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Forge {
    name: String,
}

impl Forge {
    pub fn new() -> Forge {
        Forge {
            name: "Forge".to_string(),
        }
    }
}
#[derive(Component, Debug)]
pub struct Furnace {
    name: String,
}

impl Furnace {
    pub fn new() -> Furnace {
        Furnace {
            name: "Furnace".to_string(),
        }
    }
}
#[derive(Component, Debug)]
pub struct Foundry {
    name: String,
}

impl Foundry {
    pub fn new() -> Foundry {
        Foundry {
            name: "Foundry".to_string(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Settlement {
    name: String,
    population: u32,
    asset_id: String,
}

impl Settlement {
    pub fn new() -> Settlement {
        Settlement {
            name: "Settlement".to_string(),
            asset_id: format!("hexagon-pack/PNG/Tiles/Medieval/medieval_cabin.png"),
            population: 3000,
        }
    }
}

#[derive(Component, Debug)]
pub struct Storage {
    name: String,
    owner: String,
}

impl Storage {
    pub fn new(god: String) -> Storage {
        info!("Spawning Storage for {god}");
        Storage {
            name: String::from("Storage"),
            owner: god
        }

    }
}