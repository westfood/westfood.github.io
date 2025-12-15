use bevy::{prelude::*, transform::commands};

#[derive(Component, Debug)]
pub struct IronYeald {
    pub value: u32,
}
pub struct Iron {
    pub value: u32,
}
#[derive(Component, Debug)]
pub struct Copper {
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
pub struct Tin {
    pub value: u32,
}
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
}

#[derive(Component, Debug)]
pub struct Mine {
    name: String,
    asset_id: String,
}

impl Mine {
    pub fn new(mine_type: MineTypes) -> Mine {
        if mine_type.is_valid() {
            info!("Spawning {}", mine_type.name());
            {
                Mine {
                    name: mine_type.name(),
                    asset_id: format!("hexagon-pack/PNG/Tiles/Medieval/medieval_mine.png"),
                }
            }
        } else {
            panic!("Wrong Mine Type")
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
}
