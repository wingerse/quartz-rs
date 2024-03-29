pub mod item_id;
pub mod item_stack;
//pub mod inventory;

use std::any::Any;
use std::fmt::Debug;

use crate::block::block_entity::BlockEntity;
use crate::block::{Block, BlockStateId};
use crate::nbt;

pub trait Item: Debug {
    fn get_id(&self) -> u16;
    fn get_name(&self) -> &'static str;
    fn get_damage_value(&self) -> i16 {
        0
    }
    fn update_tag(&self, tag: &mut nbt::Compound) {}
}

#[derive(Debug)]
pub struct BlockItem {
    pub id: BlockStateId,
    pub block_entity: Option<Box<dyn BlockEntity>>,
    pub can_place_on: Option<Vec<Block>>,
}

impl Item for BlockItem {
    fn get_id(&self) -> u16 {
        self.id.get_type().to_u8() as u16
    }
    fn get_name(&self) -> &'static str {
        self.id.get_type().to_name()
    }
    fn get_damage_value(&self) -> i16 {
        self.id.get_meta() as i16
    }

    fn update_tag(&self, tag: &mut nbt::Compound) {
        if let Some(ref can_place_on) = self.can_place_on {
            let mut list = nbt::List(Vec::new());
            for block in can_place_on {
                list.0.push(block.to_name().to_string().into());
            }
            tag.0.insert("CanPlaceOn".into(), list.into());
        }
        if let Some(ref block_entity) = self.block_entity {
            tag.0
                .insert("BlockEntityTag".into(), block_entity.to_nbt().into());
        }
    }
}

/*
#[derive(Debug)]
pub enum Item {
    Block(Block),
    IronShovel,
    IronPickaxe,
    IronAxe,
    FlintAndSteel,
    Apple,
    Bow,
    Arrow,
    Coal,
    Diamond,
    IronIngot,
    GoldIngot,
    IronSword,
    WoodenSword,
    WoodenShovel,
    WoodenPickaxe,
    WoodenAxe,
    StoneSword,
    StoneShovel,
    StonePickaxe,
    StoneAxe,
    DiamondSword,
    DiamondShovel,
    DiamondPickaxe,
    DiamondAxe,
    Stick,
    Bowl,
    MushroomStew,
    GoldenSword,
    GoldenShovel,
    GoldenPickaxe,
    GoldenAxe,
    String,
    Feather,
    Gunpowder,
    WoodenHoe,
    StoneHoe,
    IronHoe,
    DiamondHoe,
    GoldenHoe,
    WheatSeeds,
    Wheat,
    Bread,
    LeatherHelmet,
    LeatherChestplate,
    LeatherLeggings,
    LeatherBoots,
    ChainmailHelmet,
    ChainmailChestplate,
    ChainmailLeggings,
    ChainmailBoots,
    IronHelmet,
    IronChestplate,
    IronLeggings,
    IronBoots,
    DiamondHelmet,
    DiamondChestplate,
    DiamondLeggings,
    DiamondBoots,
    GoldenHelmet,
    GoldenChestplate,
    GoldenLeggings,
    GoldenBoots,
    Flint,
    Porkchop,
    CookedPorkchop,
    Painting,
    GoldenApple,
    Sign,
    WoodenDoor,
    Bucket,
    WaterBucket,
    LavaBucket,
    Minecart,
    Saddle,
    IronDoor,
    Redstone,
    Snowball,
    Boat,
    Leather,
    MilkBucket,
    Brick,
    ClayBall,
    Reeds,
    Paper,
    Book,
    SlimeBall,
    ChestMinecart,
    FurnaceMinecart,
    Egg,
    Compass,
    FishingRod,
    Clock,
    GlowstoneDust,
    Fish,
    CookedFish,
    Dye,
    Bone,
    Sugar,
    Cake,
    Bed,
    Repeater,
    Cookie,
    FilledMap,
    Shears,
    Melon,
    PumpkinSeeds,
    MelonSeeds,
    Beef,
    CookedBeef,
    Chicken,
    CookedChicken,
    RottenFlesh,
    EnderPearl,
    BlazeRod,
    GhastTear,
    GoldNugget,
    NetherWart,
    Potion,
    GlassBottle,
    SpiderEye,
    FermentedSpiderEye,
    BlazePowder,
    MagmaCream,
    BrewingStand,
    Cauldron,
    EnderEye,
    SpeckledMelon,
    SpawnEgg,
    ExperienceBottle,
    FireCharge,
    WritableBook,
    WrittenBook,
    Emerald,
    ItemFrame,
    FlowerPot,
    Carrot,
    Potato,
    BakedPotato,
    PoisonousPotato,
    Map,
    GoldenCarrot,
    Skull,
    CarrotOnAStick,
    NetherStar,
    PumpkinPie,
    Fireworks,
    FireworkCharge,
    EnchantedBook,
    Comparator,
    Netherbrick,
    Quartz,
    TntMinecart,
    HopperMinecart,
    PrismarineShard,
    PrismarineCrystals,
    Rabbit,
    CookedRabbit,
    RabbitStew,
    RabbitFoot,
    RabbitHide,
    ArmorStand,
    IronHorseArmor,
    GoldenHorseArmor,
    DiamondHorseArmor,
    Lead,
    NameTag,
    CommandBlockMinecart,
    Mutton,
    CookedMutton,
    Banner,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    DarkOakDoor,
    Record13,
    RecordCat,
    RecordBlocks,
    RecordChirp,
    RecordFar,
    RecordMall,
    RecordMellohi,
    RecordStal,
    RecordStrad,
    RecordWard,
    Record11,
    RecordWait,
}*/
