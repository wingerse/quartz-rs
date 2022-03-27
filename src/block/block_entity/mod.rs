mod banner_pattern;
mod dye_color;

use crate::item::item_id::PISTON;

pub use self::banner_pattern::*;
pub use self::dye_color::*;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

use serde_json;
use uuid::Uuid;

use crate::block::{Block, BlockPos, BlockStateId, Facing};
use crate::entity::player::PlayerProfile;
use crate::entity::status_effect::StatusEffect;
use crate::item::item_stack::ItemStack;
use crate::nbt::{Compound, DeserializeError, List, Tag};
use crate::text::chat::Chat;

const FURNACE_ID: &'static str = "Furnance";
const CHEST_ID: &'static str = "Chest";
const ENDER_CHEST_ID: &'static str = "EnderChest";
const JUKEBOX_ID: &'static str = "RecordPlayer";
const DISPENSER_ID: &'static str = "Trap";
const DROPPER_ID: &'static str = "Dropper";
const SIGN_ID: &'static str = "Sign";
const MOB_SPAWNER_ID: &'static str = "MobSpawner";
const NOTEBLOCK_ID: &'static str = "Music";
const PISTON_ID: &'static str = "Piston";
const BREWING_STAND_ID: &'static str = "Cauldron";
const ENCHANTMENT_TABLE_ID: &'static str = "EnchantTable";
const END_PORTAL_ID: &'static str = "Airportal";
const COMMAND_BLOCK_ID: &'static str = "Control";
const BEACON_ID: &'static str = "Beacon";
const SKULL_ID: &'static str = "Skull";
const DAYLIGHT_DETECTOR_ID: &'static str = "DLDetector";
const HOPPER_ID: &'static str = "Hopper";
const COMPARATOR_ID: &'static str = "Comparator";
const FLOWER_POT_ID: &'static str = "FlowerPot";
const BANNER_ID: &'static str = "Banner";

pub trait BlockEntity: Debug {
    fn to_nbt(&self) -> Compound;
    fn id(&self) -> &'static str;
}

#[derive(Debug, Default)]
pub struct ContainerHeader {
    pub custom_name: Option<String>,
    pub lock: Option<String>,
}

impl ContainerHeader {
    fn update_nbt(&self, temp: &mut Compound) {
        if let Some(ref custom_name) = self.custom_name {
            temp.0
                .insert("CustomName".into(), custom_name.clone().into());
        }
        if let Some(ref lock) = self.lock {
            temp.0.insert("Lock".into(), lock.clone().into());
        }
    }

    fn from_nbt(compound: &mut Compound) -> Result<ContainerHeader, DeserializeError> {
        let custom_name = if compound.contains_key("CustomName") {
            Some(compound.get("CustomName")?.as_string()?.to_string())
        } else {
            None
        };
        let lock = if compound.contains_key("Lock") {
            Some(compound.get("Lock")?.as_string()?.to_string())
        } else {
            None
        };
        Ok(ContainerHeader { custom_name, lock })
    }
}

#[derive(Debug, Default)]
pub struct Furnace {
    pub header: ContainerHeader,
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
    pub smelting_item: Option<ItemStack>,
    pub fuel: Option<ItemStack>,
    pub result: Option<ItemStack>,
}

impl BlockEntity for Furnace {
    fn to_nbt(&self) -> Compound {
        let mut compound = Compound(HashMap::new());
        self.header.update_nbt(&mut compound);
        compound.0.insert("BurnTime".into(), self.burn_time.into());
        compound.0.insert("CookTime".into(), self.cook_time.into());
        compound
            .0
            .insert("CookTimeTotal".into(), self.cook_time_total.into());
        let mut items = List(Vec::new());
        if let Some(ref smelting_item) = self.smelting_item {
            items.0.push(smelting_item.to_nbt_with_slot(0).into());
        }
        if let Some(ref fuel) = self.fuel {
            items.0.push(fuel.to_nbt_with_slot(1).into());
        }
        if let Some(ref result) = self.result {
            items.0.push(result.to_nbt_with_slot(2).into());
        }
        compound
    }

    fn id(&self) -> &'static str {
        FURNACE_ID
    }
}

#[derive(Debug, Default)]
pub struct Chest {
    pub header: ContainerHeader,
    pub items: [Option<ItemStack>; 27],
}

impl BlockEntity for Chest {
    fn to_nbt(&self) -> Compound {
        let mut compound = Compound(HashMap::new());
        self.header.update_nbt(&mut compound);
        let mut list = List(Vec::new());
        for (i, item) in self.items.iter().enumerate() {
            if let Some(ref item) = *item {
                list.0.push(item.to_nbt_with_slot(i as u8).into());
            }
        }
        compound.0.insert("Items".into(), list.into());
        compound
    }

    fn id(&self) -> &'static str {
        CHEST_ID
    }
}

#[derive(Debug, Default)]
pub struct EnderChest;

impl BlockEntity for EnderChest {
    fn to_nbt(&self) -> Compound {
        Compound(HashMap::new())
    }

    fn id(&self) -> &'static str {
        ENDER_CHEST_ID
    }
}

#[derive(Debug, Default)]
pub struct Jukebox {
    pub record: Option<ItemStack>,
}

impl BlockEntity for Jukebox {
    fn to_nbt(&self) -> Compound {
        let mut compound = Compound(HashMap::new());
        if let Some(ref record) = self.record {
            compound
                .0
                .insert("RecordItem".into(), record.to_nbt().into());
        }
        compound
    }

    fn id(&self) -> &'static str {
        JUKEBOX_ID
    }
}

#[derive(Debug, Default)]
pub struct Dispenser {
    pub header: ContainerHeader,
    pub items: [Option<ItemStack>; 9],
}

impl BlockEntity for Dispenser {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        let mut items = List(Vec::new());
        for (i, item) in self.items.iter().enumerate() {
            if let Some(ref item) = *item {
                items.0.push(item.to_nbt_with_slot(i as u8).into());
            }
        }
        com.0.insert("Items".into(), items.into());
        com
    }

    fn id(&self) -> &'static str {
        DISPENSER_ID
    }
}

#[derive(Debug, Default)]
pub struct Dropper(Dispenser);

impl BlockEntity for Dropper {
    fn to_nbt(&self) -> Compound {
        self.0.to_nbt()
    }

    fn id(&self) -> &'static str {
        DROPPER_ID
    }
}

#[derive(Debug, Default)]
pub struct Sign {
    pub text1: Chat,
    pub text2: Chat,
    pub text3: Chat,
    pub text4: Chat,
}

impl BlockEntity for Sign {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert(
            "Text1".into(),
            serde_json::to_string(&self.text1).unwrap().into(),
        );
        com.0.insert(
            "Text2".into(),
            serde_json::to_string(&self.text2).unwrap().into(),
        );
        com.0.insert(
            "Text3".into(),
            serde_json::to_string(&self.text3).unwrap().into(),
        );
        com.0.insert(
            "Text4".into(),
            serde_json::to_string(&self.text4).unwrap().into(),
        );
        com
    }

    fn id(&self) -> &'static str {
        SIGN_ID
    }
}

// not supported
#[derive(Debug, Default)]
pub struct MobSpawner;

impl BlockEntity for MobSpawner {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert("Delay".into(), 0i16.into());
        com.0.insert("EntityId".into(), "Pig".to_string().into());
        com.0.insert("MaxNearbyEntities".into(), 6i16.into());
        com.0.insert("MaxSpawnDelay".into(), 800i16.into());
        com.0.insert("MinSpawnDelay".into(), 200i16.into());
        com.0.insert("RequiredPlayerRange".into(), 16i16.into());
        com.0.insert("SpawnCount".into(), 4i16.into());
        com.0.insert("SpawnRange".into(), 4i16.into());
        com
    }

    fn id(&self) -> &'static str {
        MOB_SPAWNER_ID
    }
}

#[derive(Debug, Default)]
pub struct Noteblock {
    pub note: i8,
}

impl BlockEntity for Noteblock {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert("note".into(), self.note.into());
        com
    }

    fn id(&self) -> &'static str {
        NOTEBLOCK_ID
    }
}

#[derive(Debug, Default)]
pub struct Piston {
    pub block: BlockStateId,
    pub facing: Facing,
    pub progress: f32,
    pub extending: bool,
}

impl BlockEntity for Piston {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert(
            "blockId".into(),
            (self.block.get_type().to_u8() as i32).into(),
        );
        com.0
            .insert("blockData".into(), (self.block.get_meta() as i32).into());
        com.0
            .insert("facing".into(), (self.facing.to_i8() as i32).into());
        com.0.insert("progress".into(), self.progress.into());
        com.0.insert("extending".into(), self.extending.into());
        com
    }

    fn id(&self) -> &'static str {
        PISTON_ID
    }
}

#[derive(Debug, Default)]
pub struct BrewingStand {
    pub header: ContainerHeader,
    pub left: Option<ItemStack>,
    pub middle: Option<ItemStack>,
    pub right: Option<ItemStack>,
    pub ingredient: Option<ItemStack>,
    pub brew_time: i16,
}

impl BlockEntity for BrewingStand {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        self.header.update_nbt(&mut com);
        com.0.insert("BrewTime".into(), self.brew_time.into());
        let mut items = List(Vec::new());
        if let Some(ref left) = self.left {
            items.0.push(left.to_nbt_with_slot(0).into());
        }
        if let Some(ref middle) = self.middle {
            items.0.push(middle.to_nbt_with_slot(1).into());
        }
        if let Some(ref right) = self.right {
            items.0.push(right.to_nbt_with_slot(2).into());
        }
        if let Some(ref ingredient) = self.ingredient {
            items.0.push(ingredient.to_nbt_with_slot(3).into());
        }
        com.0.insert("Items".into(), items.into());
        com
    }

    fn id(&self) -> &'static str {
        BREWING_STAND_ID
    }
}

#[derive(Debug, Default)]
pub struct EnchantmentTable {
    pub custom_name: Option<String>,
}

impl BlockEntity for EnchantmentTable {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        if let Some(ref custom_name) = self.custom_name {
            com.0
                .insert("CustomName".into(), custom_name.clone().into());
        }
        com
    }

    fn id(&self) -> &'static str {
        ENCHANTMENT_TABLE_ID
    }
}

#[derive(Debug, Default)]
pub struct EndPortal;

impl BlockEntity for EndPortal {
    fn to_nbt(&self) -> Compound {
        Compound(HashMap::new())
    }

    fn id(&self) -> &'static str {
        END_PORTAL_ID
    }
}

#[derive(Debug, Default)]
pub struct CommandBlock;

impl BlockEntity for CommandBlock {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert("Command".into(), "".to_string().into());
        com.0.insert("CustomName".into(), "@".to_string().into());
        com.0.insert("SuccessCount".into(), 0i32.into());
        com.0.insert("TrackOutput".into(), 1i8.into());
        com
    }

    fn id(&self) -> &'static str {
        COMMAND_BLOCK_ID
    }
}

#[derive(Debug)]
pub struct Beacon {
    lock: Option<String>,
    primary: Option<StatusEffect>,
    secondary: Option<StatusEffect>,
    levels: i32,
}

impl Default for Beacon {
    fn default() -> Self {
        Beacon {
            lock: None,
            primary: None,
            secondary: None,
            levels: -1,
        }
    }
}

impl BlockEntity for Beacon {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        if let Some(ref lock) = self.lock {
            com.0.insert("Lock".into(), lock.clone().into());
        }
        if let Some(primary) = self.primary {
            com.0.insert("Primary".into(), primary.to_i32().into());
        }
        if let Some(secondary) = self.secondary {
            com.0.insert("Secondary".into(), secondary.to_i32().into());
        }
        com.0.insert("Levels".into(), self.levels.into());
        com
    }

    fn id(&self) -> &'static str {
        BEACON_ID
    }
}

#[derive(Debug, Default)]
pub struct Skull {
    pub skull_type: i8,
    pub rot: i8,
    pub profile: Option<PlayerProfile>,
}

impl BlockEntity for Skull {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert("SkullType".into(), self.skull_type.into());
        com.0.insert("Rot".into(), self.rot.into());
        if let Some(ref profile) = self.profile {
            let mut properties = Compound(HashMap::new());
            properties
                .0
                .insert("Id".into(), profile.id.0.simple().to_string().into());
            properties
                .0
                .insert("Name".into(), profile.name.clone().into());
            let mut textures = List(Vec::new());
            for p in &profile.properties {
                if p.name == "textures" {
                    let mut entry = Compound(HashMap::new());
                    entry.0.insert(
                        "Value".into(),
                        serde_json::to_string(&p.value).unwrap().into(),
                    );
                    break;
                }
            }
            properties.0.insert("textures".into(), textures.into());
            com.0.insert("Properties".into(), properties.into());
        }
        com
    }

    fn id(&self) -> &'static str {
        SKULL_ID
    }
}

#[derive(Debug, Default)]
pub struct DaylightDetector;

impl BlockEntity for DaylightDetector {
    fn to_nbt(&self) -> Compound {
        Compound(HashMap::new())
    }

    fn id(&self) -> &'static str {
        DAYLIGHT_DETECTOR_ID
    }
}

#[derive(Debug, Default)]
pub struct Hopper {
    pub header: ContainerHeader,
    pub items: [Option<ItemStack>; 5],
    pub transfer_cooldown: i32,
}

impl BlockEntity for Hopper {
    fn to_nbt(&self) -> Compound {
        let mut compound = Compound(HashMap::new());
        self.header.update_nbt(&mut compound);
        let mut list = List(Vec::new());
        for (i, item) in self.items.iter().enumerate() {
            if let Some(ref item) = *item {
                list.0.push(item.to_nbt_with_slot(i as u8).into());
            }
        }
        compound.0.insert("Items".into(), list.into());
        compound
            .0
            .insert("TransferCooldown".into(), self.transfer_cooldown.into());
        compound
    }

    fn id(&self) -> &'static str {
        HOPPER_ID
    }
}

#[derive(Debug, Default)]
pub struct Comparator {
    pub output_signal: i32,
}

impl BlockEntity for Comparator {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0
            .insert("OutputSignal".into(), self.output_signal.into());
        com
    }

    fn id(&self) -> &'static str {
        COMPARATOR_ID
    }
}

#[derive(Debug, Default)]
pub struct FlowerPot {
    pub item: BlockStateId,
}

impl BlockEntity for FlowerPot {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert(
            "Item".into(),
            self.item.get_type().to_name().to_string().into(),
        );
        com.0
            .insert("Data".into(), (self.item.get_meta() as i32).into());
        com
    }

    fn id(&self) -> &'static str {
        FLOWER_POT_ID
    }
}

#[derive(Debug, Default)]
pub struct Banner {
    pub base: DyeColor,
    pub patterns: Vec<(DyeColor, BannerPattern)>,
}

impl BlockEntity for Banner {
    fn to_nbt(&self) -> Compound {
        let mut com = Compound(HashMap::new());
        com.0.insert("Base".into(), self.base.to_i32().into());
        let mut patterns = List(Vec::new());
        for (color, pattern) in &self.patterns {
            let mut temp = Compound(HashMap::new());
            temp.0.insert("Color".into(), color.to_i32().into());
            temp.0
                .insert("Pattern".into(), pattern.to_name().to_string().into());
            patterns.0.push(temp.into());
        }
        com.0.insert("Patterns".into(), patterns.into());
        com
    }

    fn id(&self) -> &'static str {
        BANNER_ID
    }
}
