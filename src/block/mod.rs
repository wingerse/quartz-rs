pub mod block_entity;
mod facing;
mod block_pos;
mod block_state_id;

pub use self::facing::Facing;
pub use self::block_pos::BlockPos;
pub use self::block_state_id::BlockStateId;

use std::io::{self, Read, Write};

use binary;
use proto;
use sound::{self, Sound};
use item::item_id;
use self::block_entity::BlockEntity;
use entity::player::Player;
use server::ServerContext;

pub enum Air {}

pub enum Stone {}

pub enum Grass {}

pub enum Dirt {}

pub enum Cobblestone {}

pub enum Planks {}

pub enum Sapling {}

pub enum Bedrock {}

pub enum FlowingWater {}

pub enum Water {}

pub enum FlowingLava {}

pub enum Lava {}

pub enum Sand {}

pub enum Gravel {}

pub enum GoldOre {}

pub enum IronOre {}

pub enum CoalOre {}

pub enum Log {}

pub enum Leaves {}

pub enum Sponge {}

pub enum Glass {}

pub enum LapisOre {}

pub enum LapisBlock {}

pub enum Dispenser {}

pub enum Sandstone {}

pub enum Noteblock {}

pub enum Bed {}

pub enum GoldenRail {}

pub enum DetectorRail {}

pub enum StickyPiston {}

pub enum Web {}

pub enum Tallgrass {}

pub enum Deadbush {}

pub enum Piston {}

pub enum PistonHead {}

pub enum Wool {}

pub enum PistonExtension {}

pub enum YellowFlower {}

pub enum RedFlower {}

pub enum BrownMushroom {}

pub enum RedMushroom {}

pub enum GoldBlock {}

pub enum IronBlock {}

pub enum DoubleStoneSlab {}

pub enum StoneSlab {}

pub enum BrickBlock {}

pub enum Tnt {}

pub enum Bookshelf {}

pub enum MossyCobblestone {}

pub enum Obsidian {}

pub enum Torch {}

pub enum Fire {}

pub enum MobSpawner {}

pub enum OakStairs {}

pub enum Chest {}

pub enum RedstoneWire {}

pub enum DiamondOre {}

pub enum DiamondBlock {}

pub enum CraftingTable {}

pub enum Wheat {}

pub enum Farmland {}

pub enum Furnace {}

pub enum LitFurnace {}

pub enum StandingSign {}

pub enum WoodenDoor {}

pub enum Ladder {}

pub enum Rail {}

pub enum StoneStairs {}

pub enum WallSign {}

pub enum Lever {}

pub enum StonePressurePlate {}

pub enum IronDoor {}

pub enum WoodenPressurePlate {}

pub enum RedstoneOre {}

pub enum LitRedstoneOre {}

pub enum UnlitRedstoneTorch {}

pub enum RedstoneTorch {}

pub enum StoneButton {}

pub enum SnowLayer {}

pub enum Ice {}

pub enum Snow {}

pub enum Cactus {}

pub enum Clay {}

pub enum Reeds {}

pub enum Jukebox {}

pub enum Fence {}

pub enum Pumpkin {}

pub enum Netherrack {}

pub enum SoulSand {}

pub enum Glowstone {}

pub enum Portal {}

pub enum LitPumpkin {}

pub enum Cake {}

pub enum UnpoweredRepeater {}

pub enum PoweredRepeater {}

pub enum StainedGlass {}

pub enum Trapdoor {}

pub enum MonsterEgg {}

pub enum Stonebrick {}

pub enum BrownMushroomBlock {}

pub enum RedMushroomBlock {}

pub enum IronBars {}

pub enum GlassPane {}

pub enum MelonBlock {}

pub enum PumpkinStem {}

pub enum MelonStem {}

pub enum Vine {}

pub enum FenceGate {}

pub enum BrickStairs {}

pub enum StoneBrickStairs {}

pub enum Mycelium {}

pub enum Waterlily {}

pub enum NetherBrick {}

pub enum NetherBrickFence {}

pub enum NetherBrickStairs {}

pub enum NetherWart {}

pub enum EnchantingTable {}

pub enum BrewingStand {}

pub enum Cauldron {}

pub enum EndPortal {}

pub enum EndPortalFrame {}

pub enum EndStone {}

pub enum DragonEgg {}

pub enum RedstoneLamp {}

pub enum LitRedstoneLamp {}

pub enum DoubleWoodenSlab {}

pub enum WoodenSlab {}

pub enum Cocoa {}

pub enum SandstoneStairs {}

pub enum EmeraldOre {}

pub enum EnderChest {}

pub enum TripwireHook {}

pub enum Tripwire {}

pub enum EmeraldBlock {}

pub enum SpruceStairs {}

pub enum BirchStairs {}

pub enum JungleStairs {}

pub enum CommandBlock {}

pub enum Beacon {}

pub enum CobblestoneWall {}

pub enum FlowerPot {}

pub enum Carrots {}

pub enum Potatoes {}

pub enum WoodenButton {}

pub enum Skull {}

pub enum Anvil {}

pub enum TrappedChest {}

pub enum LightWeightedPressurePlate {}

pub enum HeavyWeightedPressurePlate {}

pub enum UnpoweredComparator {}

pub enum PoweredComparator {}

pub enum DaylightDetector {}

pub enum RedstoneBlock {}

pub enum QuartzOre {}

pub enum Hopper {}

pub enum QuartzBlock {}

pub enum QuartzStairs {}

pub enum ActivatorRail {}

pub enum Dropper {}

pub enum StainedHardenedClay {}

pub enum StainedGlassPane {}

pub enum Leaves2 {}

pub enum Log2 {}

pub enum AcaciaStairs {}

pub enum DarkOakStairs {}

pub enum Slime {}

pub enum Barrier {}

pub enum IronTrapdoor {}

pub enum Prismarine {}

pub enum SeaLantern {}

pub enum HayBlock {}

pub enum Carpet {}

pub enum HardenedClay {}

pub enum CoalBlock {}

pub enum PackedIce {}

pub enum DoublePlant {}

pub enum StandingBanner {}

pub enum WallBanner {}

pub enum DaylightDetectorInverted {}

pub enum RedSandstone {}

pub enum RedSandstoneStairs {}

pub enum DoubleStoneSlab2 {}

pub enum StoneSlab2 {}

pub enum SpruceFenceGate {}

pub enum BirchFenceGate {}

pub enum JungleFenceGate {}

pub enum DarkOakFenceGate {}

pub enum AcaciaFenceGate {}

pub enum SpruceFence {}

pub enum BirchFence {}

pub enum JungleFence {}

pub enum DarkOakFence {}

pub enum AcaciaFence {}

pub enum SpruceDoor {}

pub enum BirchDoor {}

pub enum JungleDoor {}

pub enum AcaciaDoor {}

pub enum DarkOakDoor {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Stone,
    Grass,
    Dirt,
    Cobblestone,
    Planks,
    Sapling,
    Bedrock,
    FlowingWater,
    Water,
    FlowingLava,
    Lava,
    Sand,
    Gravel,
    GoldOre,
    IronOre,
    CoalOre,
    Log,
    Leaves,
    Sponge,
    Glass,
    LapisOre,
    LapisBlock,
    Dispenser,
    Sandstone,
    Noteblock,
    Bed,
    GoldenRail,
    DetectorRail,
    StickyPiston,
    Web,
    Tallgrass,
    Deadbush,
    Piston,
    PistonHead,
    Wool,
    PistonExtension,
    YellowFlower,
    RedFlower,
    BrownMushroom,
    RedMushroom,
    GoldBlock,
    IronBlock,
    DoubleStoneSlab,
    StoneSlab,
    BrickBlock,
    Tnt,
    Bookshelf,
    MossyCobblestone,
    Obsidian,
    Torch,
    Fire,
    MobSpawner,
    OakStairs,
    Chest,
    RedstoneWire,
    DiamondOre,
    DiamondBlock,
    CraftingTable,
    Wheat,
    Farmland,
    Furnace,
    LitFurnace,
    StandingSign,
    WoodenDoor,
    Ladder,
    Rail,
    StoneStairs,
    WallSign,
    Lever,
    StonePressurePlate,
    IronDoor,
    WoodenPressurePlate,
    RedstoneOre,
    LitRedstoneOre,
    UnlitRedstoneTorch,
    RedstoneTorch,
    StoneButton,
    SnowLayer,
    Ice,
    Snow,
    Cactus,
    Clay,
    Reeds,
    Jukebox,
    Fence,
    Pumpkin,
    Netherrack,
    SoulSand,
    Glowstone,
    Portal,
    LitPumpkin,
    Cake,
    UnpoweredRepeater,
    PoweredRepeater,
    StainedGlass,
    Trapdoor,
    MonsterEgg,
    Stonebrick,
    BrownMushroomBlock,
    RedMushroomBlock,
    IronBars,
    GlassPane,
    MelonBlock,
    PumpkinStem,
    MelonStem,
    Vine,
    FenceGate,
    BrickStairs,
    StoneBrickStairs,
    Mycelium,
    Waterlily,
    NetherBrick,
    NetherBrickFence,
    NetherBrickStairs,
    NetherWart,
    EnchantingTable,
    BrewingStand,
    Cauldron,
    EndPortal,
    EndPortalFrame,
    EndStone,
    DragonEgg,
    RedstoneLamp,
    LitRedstoneLamp,
    DoubleWoodenSlab,
    WoodenSlab,
    Cocoa,
    SandstoneStairs,
    EmeraldOre,
    EnderChest,
    TripwireHook,
    Tripwire,
    EmeraldBlock,
    SpruceStairs,
    BirchStairs,
    JungleStairs,
    CommandBlock,
    Beacon,
    CobblestoneWall,
    FlowerPot,
    Carrots,
    Potatoes,
    WoodenButton,
    Skull,
    Anvil,
    TrappedChest,
    LightWeightedPressurePlate,
    HeavyWeightedPressurePlate,
    UnpoweredComparator,
    PoweredComparator,
    DaylightDetector,
    RedstoneBlock,
    QuartzOre,
    Hopper,
    QuartzBlock,
    QuartzStairs,
    ActivatorRail,
    Dropper,
    StainedHardenedClay,
    StainedGlassPane,
    Leaves2,
    Log2,
    AcaciaStairs,
    DarkOakStairs,
    Slime,
    Barrier,
    IronTrapdoor,
    Prismarine,
    SeaLantern,
    HayBlock,
    Carpet,
    HardenedClay,
    CoalBlock,
    PackedIce,
    DoublePlant,
    StandingBanner,
    WallBanner,
    DaylightDetectorInverted,
    RedSandstone,
    RedSandstoneStairs,
    DoubleStoneSlab2,
    StoneSlab2,
    SpruceFenceGate,
    BirchFenceGate,
    JungleFenceGate,
    DarkOakFenceGate,
    AcaciaFenceGate,
    SpruceFence,
    BirchFence,
    JungleFence,
    DarkOakFence,
    AcaciaFence,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    DarkOakDoor,
}

impl Default for Block {
    fn default() -> Self { Block::Air }
}

impl Block {
    pub fn to_u8(&self) -> u8 {
        use self::Block::*;
        match *self {
            Air => item_id::AIR as u8,
            Stone => item_id::STONE as u8,
            Grass => item_id::GRASS as u8,
            Dirt => item_id::DIRT as u8,
            Cobblestone => item_id::COBBLESTONE as u8,
            Planks => item_id::PLANKS as u8,
            Sapling => item_id::SAPLING as u8,
            Bedrock => item_id::BEDROCK as u8,
            FlowingWater => item_id::FLOWING_WATER as u8,
            Water => item_id::WATER as u8,
            FlowingLava => item_id::FLOWING_LAVA as u8,
            Lava => item_id::LAVA as u8,
            Sand => item_id::SAND as u8,
            Gravel => item_id::GRAVEL as u8,
            GoldOre => item_id::GOLD_ORE as u8,
            IronOre => item_id::IRON_ORE as u8,
            CoalOre => item_id::COAL_ORE as u8,
            Log => item_id::LOG as u8,
            Leaves => item_id::LEAVES as u8,
            Sponge => item_id::SPONGE as u8,
            Glass => item_id::GLASS as u8,
            LapisOre => item_id::LAPIS_ORE as u8,
            LapisBlock => item_id::LAPIS_BLOCK as u8,
            Dispenser => item_id::DISPENSER as u8,
            Sandstone => item_id::SANDSTONE as u8,
            Noteblock => item_id::NOTEBLOCK as u8,
            Bed => item_id::BED as u8,
            GoldenRail => item_id::GOLDEN_RAIL as u8,
            DetectorRail => item_id::DETECTOR_RAIL as u8,
            StickyPiston => item_id::STICKY_PISTON as u8,
            Web => item_id::WEB as u8,
            Tallgrass => item_id::TALLGRASS as u8,
            Deadbush => item_id::DEADBUSH as u8,
            Piston => item_id::PISTON as u8,
            PistonHead => item_id::PISTON_HEAD as u8,
            Wool => item_id::WOOL as u8,
            PistonExtension => item_id::PISTON_EXTENSION as u8,
            YellowFlower => item_id::YELLOW_FLOWER as u8,
            RedFlower => item_id::RED_FLOWER as u8,
            BrownMushroom => item_id::BROWN_MUSHROOM as u8,
            RedMushroom => item_id::RED_MUSHROOM as u8,
            GoldBlock => item_id::GOLD_BLOCK as u8,
            IronBlock => item_id::IRON_BLOCK as u8,
            DoubleStoneSlab => item_id::DOUBLE_STONE_SLAB as u8,
            StoneSlab => item_id::STONE_SLAB as u8,
            BrickBlock => item_id::BRICK_BLOCK as u8,
            Tnt => item_id::TNT as u8,
            Bookshelf => item_id::BOOKSHELF as u8,
            MossyCobblestone => item_id::MOSSY_COBBLESTONE as u8,
            Obsidian => item_id::OBSIDIAN as u8,
            Torch => item_id::TORCH as u8,
            Fire => item_id::FIRE as u8,
            MobSpawner => item_id::MOB_SPAWNER as u8,
            OakStairs => item_id::OAK_STAIRS as u8,
            Chest => item_id::CHEST as u8,
            RedstoneWire => item_id::REDSTONE_WIRE as u8,
            DiamondOre => item_id::DIAMOND_ORE as u8,
            DiamondBlock => item_id::DIAMOND_BLOCK as u8,
            CraftingTable => item_id::CRAFTING_TABLE as u8,
            Wheat => item_id::WHEAT as u8,
            Farmland => item_id::FARMLAND as u8,
            Furnace => item_id::FURNACE as u8,
            LitFurnace => item_id::LIT_FURNACE as u8,
            StandingSign => item_id::STANDING_SIGN as u8,
            WoodenDoor => item_id::WOODEN_DOOR as u8,
            Ladder => item_id::LADDER as u8,
            Rail => item_id::RAIL as u8,
            StoneStairs => item_id::STONE_STAIRS as u8,
            WallSign => item_id::WALL_SIGN as u8,
            Lever => item_id::LEVER as u8,
            StonePressurePlate => item_id::STONE_PRESSURE_PLATE as u8,
            IronDoor => item_id::IRON_DOOR as u8,
            WoodenPressurePlate => item_id::WOODEN_PRESSURE_PLATE as u8,
            RedstoneOre => item_id::REDSTONE_ORE as u8,
            LitRedstoneOre => item_id::LIT_REDSTONE_ORE as u8,
            UnlitRedstoneTorch => item_id::UNLIT_REDSTONE_TORCH as u8,
            RedstoneTorch => item_id::REDSTONE_TORCH as u8,
            StoneButton => item_id::STONE_BUTTON as u8,
            SnowLayer => item_id::SNOW_LAYER as u8,
            Ice => item_id::ICE as u8,
            Snow => item_id::SNOW as u8,
            Cactus => item_id::CACTUS as u8,
            Clay => item_id::CLAY as u8,
            Reeds => item_id::REEDS as u8,
            Jukebox => item_id::JUKEBOX as u8,
            Fence => item_id::FENCE as u8,
            Pumpkin => item_id::PUMPKIN as u8,
            Netherrack => item_id::NETHERRACK as u8,
            SoulSand => item_id::SOUL_SAND as u8,
            Glowstone => item_id::GLOWSTONE as u8,
            Portal => item_id::PORTAL as u8,
            LitPumpkin => item_id::LIT_PUMPKIN as u8,
            Cake => item_id::CAKE as u8,
            UnpoweredRepeater => item_id::UNPOWERED_REPEATER as u8,
            PoweredRepeater => item_id::POWERED_REPEATER as u8,
            StainedGlass => item_id::STAINED_GLASS as u8,
            Trapdoor => item_id::TRAPDOOR as u8,
            MonsterEgg => item_id::MONSTER_EGG as u8,
            Stonebrick => item_id::STONEBRICK as u8,
            BrownMushroomBlock => item_id::BROWN_MUSHROOM_BLOCK as u8,
            RedMushroomBlock => item_id::RED_MUSHROOM_BLOCK as u8,
            IronBars => item_id::IRON_BARS as u8,
            GlassPane => item_id::GLASS_PANE as u8,
            MelonBlock => item_id::MELON_BLOCK as u8,
            PumpkinStem => item_id::PUMPKIN_STEM as u8,
            MelonStem => item_id::MELON_STEM as u8,
            Vine => item_id::VINE as u8,
            FenceGate => item_id::FENCE_GATE as u8,
            BrickStairs => item_id::BRICK_STAIRS as u8,
            StoneBrickStairs => item_id::STONE_BRICK_STAIRS as u8,
            Mycelium => item_id::MYCELIUM as u8,
            Waterlily => item_id::WATERLILY as u8,
            NetherBrick => item_id::NETHER_BRICK as u8,
            NetherBrickFence => item_id::NETHER_BRICK_FENCE as u8,
            NetherBrickStairs => item_id::NETHER_BRICK_STAIRS as u8,
            NetherWart => item_id::NETHER_WART as u8,
            EnchantingTable => item_id::ENCHANTING_TABLE as u8,
            BrewingStand => item_id::BREWING_STAND as u8,
            Cauldron => item_id::CAULDRON as u8,
            EndPortal => item_id::END_PORTAL as u8,
            EndPortalFrame => item_id::END_PORTAL_FRAME as u8,
            EndStone => item_id::END_STONE as u8,
            DragonEgg => item_id::DRAGON_EGG as u8,
            RedstoneLamp => item_id::REDSTONE_LAMP as u8,
            LitRedstoneLamp => item_id::LIT_REDSTONE_LAMP as u8,
            DoubleWoodenSlab => item_id::DOUBLE_WOODEN_SLAB as u8,
            WoodenSlab => item_id::WOODEN_SLAB as u8,
            Cocoa => item_id::COCOA as u8,
            SandstoneStairs => item_id::SANDSTONE_STAIRS as u8,
            EmeraldOre => item_id::EMERALD_ORE as u8,
            EnderChest => item_id::ENDER_CHEST as u8,
            TripwireHook => item_id::TRIPWIRE_HOOK as u8,
            Tripwire => item_id::TRIPWIRE as u8,
            EmeraldBlock => item_id::EMERALD_BLOCK as u8,
            SpruceStairs => item_id::SPRUCE_STAIRS as u8,
            BirchStairs => item_id::BIRCH_STAIRS as u8,
            JungleStairs => item_id::JUNGLE_STAIRS as u8,
            CommandBlock => item_id::COMMAND_BLOCK as u8,
            Beacon => item_id::BEACON as u8,
            CobblestoneWall => item_id::COBBLESTONE_WALL as u8,
            FlowerPot => item_id::FLOWER_POT as u8,
            Carrots => item_id::CARROTS as u8,
            Potatoes => item_id::POTATOES as u8,
            WoodenButton => item_id::WOODEN_BUTTON as u8,
            Skull => item_id::SKULL as u8,
            Anvil => item_id::ANVIL as u8,
            TrappedChest => item_id::TRAPPED_CHEST as u8,
            LightWeightedPressurePlate => item_id::LIGHT_WEIGHTED_PRESSURE_PLATE as u8,
            HeavyWeightedPressurePlate => item_id::HEAVY_WEIGHTED_PRESSURE_PLATE as u8,
            UnpoweredComparator => item_id::UNPOWERED_COMPARATOR as u8,
            PoweredComparator => item_id::POWERED_COMPARATOR as u8,
            DaylightDetector => item_id::DAYLIGHT_DETECTOR as u8,
            RedstoneBlock => item_id::REDSTONE_BLOCK as u8,
            QuartzOre => item_id::QUARTZ_ORE as u8,
            Hopper => item_id::HOPPER as u8,
            QuartzBlock => item_id::QUARTZ_BLOCK as u8,
            QuartzStairs => item_id::QUARTZ_STAIRS as u8,
            ActivatorRail => item_id::ACTIVATOR_RAIL as u8,
            Dropper => item_id::DROPPER as u8,
            StainedHardenedClay => item_id::STAINED_HARDENED_CLAY as u8,
            StainedGlassPane => item_id::STAINED_GLASS_PANE as u8,
            Leaves2 => item_id::LEAVES2 as u8,
            Log2 => item_id::LOG2 as u8,
            AcaciaStairs => item_id::ACACIA_STAIRS as u8,
            DarkOakStairs => item_id::DARK_OAK_STAIRS as u8,
            Slime => item_id::SLIME as u8,
            Barrier => item_id::BARRIER as u8,
            IronTrapdoor => item_id::IRON_TRAPDOOR as u8,
            Prismarine => item_id::PRISMARINE as u8,
            SeaLantern => item_id::SEA_LANTERN as u8,
            HayBlock => item_id::HAY_BLOCK as u8,
            Carpet => item_id::CARPET as u8,
            HardenedClay => item_id::HARDENED_CLAY as u8,
            CoalBlock => item_id::COAL_BLOCK as u8,
            PackedIce => item_id::PACKED_ICE as u8,
            DoublePlant => item_id::DOUBLE_PLANT as u8,
            StandingBanner => item_id::STANDING_BANNER as u8,
            WallBanner => item_id::WALL_BANNER as u8,
            DaylightDetectorInverted => item_id::DAYLIGHT_DETECTOR_INVERTED as u8,
            RedSandstone => item_id::RED_SANDSTONE as u8,
            RedSandstoneStairs => item_id::RED_SANDSTONE_STAIRS as u8,
            DoubleStoneSlab2 => item_id::DOUBLE_STONE_SLAB2 as u8,
            StoneSlab2 => item_id::STONE_SLAB2 as u8,
            SpruceFenceGate => item_id::SPRUCE_FENCE_GATE as u8,
            BirchFenceGate => item_id::BIRCH_FENCE_GATE as u8,
            JungleFenceGate => item_id::JUNGLE_FENCE_GATE as u8,
            DarkOakFenceGate => item_id::DARK_OAK_FENCE_GATE as u8,
            AcaciaFenceGate => item_id::ACACIA_FENCE_GATE as u8,
            SpruceFence => item_id::SPRUCE_FENCE as u8,
            BirchFence => item_id::BIRCH_FENCE as u8,
            JungleFence => item_id::JUNGLE_FENCE as u8,
            DarkOakFence => item_id::DARK_OAK_FENCE as u8,
            AcaciaFence => item_id::ACACIA_FENCE as u8,
            SpruceDoor => item_id::SPRUCE_DOOR as u8,
            BirchDoor => item_id::BIRCH_DOOR as u8,
            JungleDoor => item_id::JUNGLE_DOOR as u8,
            AcaciaDoor => item_id::ACACIA_DOOR as u8,
            DarkOakDoor => item_id::DARK_OAK_DOOR as u8,
        }
    }

    pub fn from_u8(byte: u8) -> Option<Block> {
        use self::Block::*;
        match byte as u16 {
            item_id::AIR => Some(Air),
            item_id::STONE => Some(Stone),
            item_id::GRASS => Some(Grass),
            item_id::DIRT => Some(Dirt),
            item_id::COBBLESTONE => Some(Cobblestone),
            item_id::PLANKS => Some(Planks),
            item_id::SAPLING => Some(Sapling),
            item_id::BEDROCK => Some(Bedrock),
            item_id::FLOWING_WATER => Some(FlowingWater),
            item_id::WATER => Some(Water),
            item_id::FLOWING_LAVA => Some(FlowingLava),
            item_id::LAVA => Some(Lava),
            item_id::SAND => Some(Sand),
            item_id::GRAVEL => Some(Gravel),
            item_id::GOLD_ORE => Some(GoldOre),
            item_id::IRON_ORE => Some(IronOre),
            item_id::COAL_ORE => Some(CoalOre),
            item_id::LOG => Some(Log),
            item_id::LEAVES => Some(Leaves),
            item_id::SPONGE => Some(Sponge),
            item_id::GLASS => Some(Glass),
            item_id::LAPIS_ORE => Some(LapisOre),
            item_id::LAPIS_BLOCK => Some(LapisBlock),
            item_id::DISPENSER => Some(Dispenser),
            item_id::SANDSTONE => Some(Sandstone),
            item_id::NOTEBLOCK => Some(Noteblock),
            item_id::BED => Some(Bed),
            item_id::GOLDEN_RAIL => Some(GoldenRail),
            item_id::DETECTOR_RAIL => Some(DetectorRail),
            item_id::STICKY_PISTON => Some(StickyPiston),
            item_id::WEB => Some(Web),
            item_id::TALLGRASS => Some(Tallgrass),
            item_id::DEADBUSH => Some(Deadbush),
            item_id::PISTON => Some(Piston),
            item_id::PISTON_HEAD => Some(PistonHead),
            item_id::WOOL => Some(Wool),
            item_id::PISTON_EXTENSION => Some(PistonExtension),
            item_id::YELLOW_FLOWER => Some(YellowFlower),
            item_id::RED_FLOWER => Some(RedFlower),
            item_id::BROWN_MUSHROOM => Some(BrownMushroom),
            item_id::RED_MUSHROOM => Some(RedMushroom),
            item_id::GOLD_BLOCK => Some(GoldBlock),
            item_id::IRON_BLOCK => Some(IronBlock),
            item_id::DOUBLE_STONE_SLAB => Some(DoubleStoneSlab),
            item_id::STONE_SLAB => Some(StoneSlab),
            item_id::BRICK_BLOCK => Some(BrickBlock),
            item_id::TNT => Some(Tnt),
            item_id::BOOKSHELF => Some(Bookshelf),
            item_id::MOSSY_COBBLESTONE => Some(MossyCobblestone),
            item_id::OBSIDIAN => Some(Obsidian),
            item_id::TORCH => Some(Torch),
            item_id::FIRE => Some(Fire),
            item_id::MOB_SPAWNER => Some(MobSpawner),
            item_id::OAK_STAIRS => Some(OakStairs),
            item_id::CHEST => Some(Chest),
            item_id::REDSTONE_WIRE => Some(RedstoneWire),
            item_id::DIAMOND_ORE => Some(DiamondOre),
            item_id::DIAMOND_BLOCK => Some(DiamondBlock),
            item_id::CRAFTING_TABLE => Some(CraftingTable),
            item_id::WHEAT => Some(Wheat),
            item_id::FARMLAND => Some(Farmland),
            item_id::FURNACE => Some(Furnace),
            item_id::LIT_FURNACE => Some(LitFurnace),
            item_id::STANDING_SIGN => Some(StandingSign),
            item_id::WOODEN_DOOR => Some(WoodenDoor),
            item_id::LADDER => Some(Ladder),
            item_id::RAIL => Some(Rail),
            item_id::STONE_STAIRS => Some(StoneStairs),
            item_id::WALL_SIGN => Some(WallSign),
            item_id::LEVER => Some(Lever),
            item_id::STONE_PRESSURE_PLATE => Some(StonePressurePlate),
            item_id::IRON_DOOR => Some(IronDoor),
            item_id::WOODEN_PRESSURE_PLATE => Some(WoodenPressurePlate),
            item_id::REDSTONE_ORE => Some(RedstoneOre),
            item_id::LIT_REDSTONE_ORE => Some(LitRedstoneOre),
            item_id::UNLIT_REDSTONE_TORCH => Some(UnlitRedstoneTorch),
            item_id::REDSTONE_TORCH => Some(RedstoneTorch),
            item_id::STONE_BUTTON => Some(StoneButton),
            item_id::SNOW_LAYER => Some(SnowLayer),
            item_id::ICE => Some(Ice),
            item_id::SNOW => Some(Snow),
            item_id::CACTUS => Some(Cactus),
            item_id::CLAY => Some(Clay),
            item_id::REEDS => Some(Reeds),
            item_id::JUKEBOX => Some(Jukebox),
            item_id::FENCE => Some(Fence),
            item_id::PUMPKIN => Some(Pumpkin),
            item_id::NETHERRACK => Some(Netherrack),
            item_id::SOUL_SAND => Some(SoulSand),
            item_id::GLOWSTONE => Some(Glowstone),
            item_id::PORTAL => Some(Portal),
            item_id::LIT_PUMPKIN => Some(LitPumpkin),
            item_id::CAKE => Some(Cake),
            item_id::UNPOWERED_REPEATER => Some(UnpoweredRepeater),
            item_id::POWERED_REPEATER => Some(PoweredRepeater),
            item_id::STAINED_GLASS => Some(StainedGlass),
            item_id::TRAPDOOR => Some(Trapdoor),
            item_id::MONSTER_EGG => Some(MonsterEgg),
            item_id::STONEBRICK => Some(Stonebrick),
            item_id::BROWN_MUSHROOM_BLOCK => Some(BrownMushroomBlock),
            item_id::RED_MUSHROOM_BLOCK => Some(RedMushroomBlock),
            item_id::IRON_BARS => Some(IronBars),
            item_id::GLASS_PANE => Some(GlassPane),
            item_id::MELON_BLOCK => Some(MelonBlock),
            item_id::PUMPKIN_STEM => Some(PumpkinStem),
            item_id::MELON_STEM => Some(MelonStem),
            item_id::VINE => Some(Vine),
            item_id::FENCE_GATE => Some(FenceGate),
            item_id::BRICK_STAIRS => Some(BrickStairs),
            item_id::STONE_BRICK_STAIRS => Some(StoneBrickStairs),
            item_id::MYCELIUM => Some(Mycelium),
            item_id::WATERLILY => Some(Waterlily),
            item_id::NETHER_BRICK => Some(NetherBrick),
            item_id::NETHER_BRICK_FENCE => Some(NetherBrickFence),
            item_id::NETHER_BRICK_STAIRS => Some(NetherBrickStairs),
            item_id::NETHER_WART => Some(NetherWart),
            item_id::ENCHANTING_TABLE => Some(EnchantingTable),
            item_id::BREWING_STAND => Some(BrewingStand),
            item_id::CAULDRON => Some(Cauldron),
            item_id::END_PORTAL => Some(EndPortal),
            item_id::END_PORTAL_FRAME => Some(EndPortalFrame),
            item_id::END_STONE => Some(EndStone),
            item_id::DRAGON_EGG => Some(DragonEgg),
            item_id::REDSTONE_LAMP => Some(RedstoneLamp),
            item_id::LIT_REDSTONE_LAMP => Some(LitRedstoneLamp),
            item_id::DOUBLE_WOODEN_SLAB => Some(DoubleWoodenSlab),
            item_id::WOODEN_SLAB => Some(WoodenSlab),
            item_id::COCOA => Some(Cocoa),
            item_id::SANDSTONE_STAIRS => Some(SandstoneStairs),
            item_id::EMERALD_ORE => Some(EmeraldOre),
            item_id::ENDER_CHEST => Some(EnderChest),
            item_id::TRIPWIRE_HOOK => Some(TripwireHook),
            item_id::TRIPWIRE => Some(Tripwire),
            item_id::EMERALD_BLOCK => Some(EmeraldBlock),
            item_id::SPRUCE_STAIRS => Some(SpruceStairs),
            item_id::BIRCH_STAIRS => Some(BirchStairs),
            item_id::JUNGLE_STAIRS => Some(JungleStairs),
            item_id::COMMAND_BLOCK => Some(CommandBlock),
            item_id::BEACON => Some(Beacon),
            item_id::COBBLESTONE_WALL => Some(CobblestoneWall),
            item_id::FLOWER_POT => Some(FlowerPot),
            item_id::CARROTS => Some(Carrots),
            item_id::POTATOES => Some(Potatoes),
            item_id::WOODEN_BUTTON => Some(WoodenButton),
            item_id::SKULL => Some(Skull),
            item_id::ANVIL => Some(Anvil),
            item_id::TRAPPED_CHEST => Some(TrappedChest),
            item_id::LIGHT_WEIGHTED_PRESSURE_PLATE => Some(LightWeightedPressurePlate),
            item_id::HEAVY_WEIGHTED_PRESSURE_PLATE => Some(HeavyWeightedPressurePlate),
            item_id::UNPOWERED_COMPARATOR => Some(UnpoweredComparator),
            item_id::POWERED_COMPARATOR => Some(PoweredComparator),
            item_id::DAYLIGHT_DETECTOR => Some(DaylightDetector),
            item_id::REDSTONE_BLOCK => Some(RedstoneBlock),
            item_id::QUARTZ_ORE => Some(QuartzOre),
            item_id::HOPPER => Some(Hopper),
            item_id::QUARTZ_BLOCK => Some(QuartzBlock),
            item_id::QUARTZ_STAIRS => Some(QuartzStairs),
            item_id::ACTIVATOR_RAIL => Some(ActivatorRail),
            item_id::DROPPER => Some(Dropper),
            item_id::STAINED_HARDENED_CLAY => Some(StainedHardenedClay),
            item_id::STAINED_GLASS_PANE => Some(StainedGlassPane),
            item_id::LEAVES2 => Some(Leaves2),
            item_id::LOG2 => Some(Log2),
            item_id::ACACIA_STAIRS => Some(AcaciaStairs),
            item_id::DARK_OAK_STAIRS => Some(DarkOakStairs),
            item_id::SLIME => Some(Slime),
            item_id::BARRIER => Some(Barrier),
            item_id::IRON_TRAPDOOR => Some(IronTrapdoor),
            item_id::PRISMARINE => Some(Prismarine),
            item_id::SEA_LANTERN => Some(SeaLantern),
            item_id::HAY_BLOCK => Some(HayBlock),
            item_id::CARPET => Some(Carpet),
            item_id::HARDENED_CLAY => Some(HardenedClay),
            item_id::COAL_BLOCK => Some(CoalBlock),
            item_id::PACKED_ICE => Some(PackedIce),
            item_id::DOUBLE_PLANT => Some(DoublePlant),
            item_id::STANDING_BANNER => Some(StandingBanner),
            item_id::WALL_BANNER => Some(WallBanner),
            item_id::DAYLIGHT_DETECTOR_INVERTED => Some(DaylightDetectorInverted),
            item_id::RED_SANDSTONE => Some(RedSandstone),
            item_id::RED_SANDSTONE_STAIRS => Some(RedSandstoneStairs),
            item_id::DOUBLE_STONE_SLAB2 => Some(DoubleStoneSlab2),
            item_id::STONE_SLAB2 => Some(StoneSlab2),
            item_id::SPRUCE_FENCE_GATE => Some(SpruceFenceGate),
            item_id::BIRCH_FENCE_GATE => Some(BirchFenceGate),
            item_id::JUNGLE_FENCE_GATE => Some(JungleFenceGate),
            item_id::DARK_OAK_FENCE_GATE => Some(DarkOakFenceGate),
            item_id::ACACIA_FENCE_GATE => Some(AcaciaFenceGate),
            item_id::SPRUCE_FENCE => Some(SpruceFence),
            item_id::BIRCH_FENCE => Some(BirchFence),
            item_id::JUNGLE_FENCE => Some(JungleFence),
            item_id::DARK_OAK_FENCE => Some(DarkOakFence),
            item_id::ACACIA_FENCE => Some(AcaciaFence),
            item_id::SPRUCE_DOOR => Some(SpruceDoor),
            item_id::BIRCH_DOOR => Some(BirchDoor),
            item_id::JUNGLE_DOOR => Some(JungleDoor),
            item_id::ACACIA_DOOR => Some(AcaciaDoor),
            item_id::DARK_OAK_DOOR => Some(DarkOakDoor),
            _ => None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        use self::Block::*;
        match *self {
            Air => item_id::AIR_NAME,
            Stone => item_id::STONE_NAME,
            Grass => item_id::GRASS_NAME,
            Dirt => item_id::DIRT_NAME,
            Cobblestone => item_id::COBBLESTONE_NAME,
            Planks => item_id::PLANKS_NAME,
            Sapling => item_id::SAPLING_NAME,
            Bedrock => item_id::BEDROCK_NAME,
            FlowingWater => item_id::FLOWING_WATER_NAME,
            Water => item_id::WATER_NAME,
            FlowingLava => item_id::FLOWING_LAVA_NAME,
            Lava => item_id::LAVA_NAME,
            Sand => item_id::SAND_NAME,
            Gravel => item_id::GRAVEL_NAME,
            GoldOre => item_id::GOLD_ORE_NAME,
            IronOre => item_id::IRON_ORE_NAME,
            CoalOre => item_id::COAL_ORE_NAME,
            Log => item_id::LOG_NAME,
            Leaves => item_id::LEAVES_NAME,
            Sponge => item_id::SPONGE_NAME,
            Glass => item_id::GLASS_NAME,
            LapisOre => item_id::LAPIS_ORE_NAME,
            LapisBlock => item_id::LAPIS_BLOCK_NAME,
            Dispenser => item_id::DISPENSER_NAME,
            Sandstone => item_id::SANDSTONE_NAME,
            Noteblock => item_id::NOTEBLOCK_NAME,
            Bed => item_id::BED_NAME,
            GoldenRail => item_id::GOLDEN_RAIL_NAME,
            DetectorRail => item_id::DETECTOR_RAIL_NAME,
            StickyPiston => item_id::STICKY_PISTON_NAME,
            Web => item_id::WEB_NAME,
            Tallgrass => item_id::TALLGRASS_NAME,
            Deadbush => item_id::DEADBUSH_NAME,
            Piston => item_id::PISTON_NAME,
            PistonHead => item_id::PISTON_HEAD_NAME,
            Wool => item_id::WOOL_NAME,
            PistonExtension => item_id::PISTON_EXTENSION_NAME,
            YellowFlower => item_id::YELLOW_FLOWER_NAME,
            RedFlower => item_id::RED_FLOWER_NAME,
            BrownMushroom => item_id::BROWN_MUSHROOM_NAME,
            RedMushroom => item_id::RED_MUSHROOM_NAME,
            GoldBlock => item_id::GOLD_BLOCK_NAME,
            IronBlock => item_id::IRON_BLOCK_NAME,
            DoubleStoneSlab => item_id::DOUBLE_STONE_SLAB_NAME,
            StoneSlab => item_id::STONE_SLAB_NAME,
            BrickBlock => item_id::BRICK_BLOCK_NAME,
            Tnt => item_id::TNT_NAME,
            Bookshelf => item_id::BOOKSHELF_NAME,
            MossyCobblestone => item_id::MOSSY_COBBLESTONE_NAME,
            Obsidian => item_id::OBSIDIAN_NAME,
            Torch => item_id::TORCH_NAME,
            Fire => item_id::FIRE_NAME,
            MobSpawner => item_id::MOB_SPAWNER_NAME,
            OakStairs => item_id::OAK_STAIRS_NAME,
            Chest => item_id::CHEST_NAME,
            RedstoneWire => item_id::REDSTONE_WIRE_NAME,
            DiamondOre => item_id::DIAMOND_ORE_NAME,
            DiamondBlock => item_id::DIAMOND_BLOCK_NAME,
            CraftingTable => item_id::CRAFTING_TABLE_NAME,
            Wheat => item_id::WHEAT_NAME,
            Farmland => item_id::FARMLAND_NAME,
            Furnace => item_id::FURNACE_NAME,
            LitFurnace => item_id::LIT_FURNACE_NAME,
            StandingSign => item_id::STANDING_SIGN_NAME,
            WoodenDoor => item_id::WOODEN_DOOR_NAME,
            Ladder => item_id::LADDER_NAME,
            Rail => item_id::RAIL_NAME,
            StoneStairs => item_id::STONE_STAIRS_NAME,
            WallSign => item_id::WALL_SIGN_NAME,
            Lever => item_id::LEVER_NAME,
            StonePressurePlate => item_id::STONE_PRESSURE_PLATE_NAME,
            IronDoor => item_id::IRON_DOOR_NAME,
            WoodenPressurePlate => item_id::WOODEN_PRESSURE_PLATE_NAME,
            RedstoneOre => item_id::REDSTONE_ORE_NAME,
            LitRedstoneOre => item_id::LIT_REDSTONE_ORE_NAME,
            UnlitRedstoneTorch => item_id::UNLIT_REDSTONE_TORCH_NAME,
            RedstoneTorch => item_id::REDSTONE_TORCH_NAME,
            StoneButton => item_id::STONE_BUTTON_NAME,
            SnowLayer => item_id::SNOW_LAYER_NAME,
            Ice => item_id::ICE_NAME,
            Snow => item_id::SNOW_NAME,
            Cactus => item_id::CACTUS_NAME,
            Clay => item_id::CLAY_NAME,
            Reeds => item_id::REEDS_NAME,
            Jukebox => item_id::JUKEBOX_NAME,
            Fence => item_id::FENCE_NAME,
            Pumpkin => item_id::PUMPKIN_NAME,
            Netherrack => item_id::NETHERRACK_NAME,
            SoulSand => item_id::SOUL_SAND_NAME,
            Glowstone => item_id::GLOWSTONE_NAME,
            Portal => item_id::PORTAL_NAME,
            LitPumpkin => item_id::LIT_PUMPKIN_NAME,
            Cake => item_id::CAKE_NAME,
            UnpoweredRepeater => item_id::UNPOWERED_REPEATER_NAME,
            PoweredRepeater => item_id::POWERED_REPEATER_NAME,
            StainedGlass => item_id::STAINED_GLASS_NAME,
            Trapdoor => item_id::TRAPDOOR_NAME,
            MonsterEgg => item_id::MONSTER_EGG_NAME,
            Stonebrick => item_id::STONEBRICK_NAME,
            BrownMushroomBlock => item_id::BROWN_MUSHROOM_BLOCK_NAME,
            RedMushroomBlock => item_id::RED_MUSHROOM_BLOCK_NAME,
            IronBars => item_id::IRON_BARS_NAME,
            GlassPane => item_id::GLASS_PANE_NAME,
            MelonBlock => item_id::MELON_BLOCK_NAME,
            PumpkinStem => item_id::PUMPKIN_STEM_NAME,
            MelonStem => item_id::MELON_STEM_NAME,
            Vine => item_id::VINE_NAME,
            FenceGate => item_id::FENCE_GATE_NAME,
            BrickStairs => item_id::BRICK_STAIRS_NAME,
            StoneBrickStairs => item_id::STONE_BRICK_STAIRS_NAME,
            Mycelium => item_id::MYCELIUM_NAME,
            Waterlily => item_id::WATERLILY_NAME,
            NetherBrick => item_id::NETHER_BRICK_NAME,
            NetherBrickFence => item_id::NETHER_BRICK_FENCE_NAME,
            NetherBrickStairs => item_id::NETHER_BRICK_STAIRS_NAME,
            NetherWart => item_id::NETHER_WART_NAME,
            EnchantingTable => item_id::ENCHANTING_TABLE_NAME,
            BrewingStand => item_id::BREWING_STAND_NAME,
            Cauldron => item_id::CAULDRON_NAME,
            EndPortal => item_id::END_PORTAL_NAME,
            EndPortalFrame => item_id::END_PORTAL_FRAME_NAME,
            EndStone => item_id::END_STONE_NAME,
            DragonEgg => item_id::DRAGON_EGG_NAME,
            RedstoneLamp => item_id::REDSTONE_LAMP_NAME,
            LitRedstoneLamp => item_id::LIT_REDSTONE_LAMP_NAME,
            DoubleWoodenSlab => item_id::DOUBLE_WOODEN_SLAB_NAME,
            WoodenSlab => item_id::WOODEN_SLAB_NAME,
            Cocoa => item_id::COCOA_NAME,
            SandstoneStairs => item_id::SANDSTONE_STAIRS_NAME,
            EmeraldOre => item_id::EMERALD_ORE_NAME,
            EnderChest => item_id::ENDER_CHEST_NAME,
            TripwireHook => item_id::TRIPWIRE_HOOK_NAME,
            Tripwire => item_id::TRIPWIRE_NAME,
            EmeraldBlock => item_id::EMERALD_BLOCK_NAME,
            SpruceStairs => item_id::SPRUCE_STAIRS_NAME,
            BirchStairs => item_id::BIRCH_STAIRS_NAME,
            JungleStairs => item_id::JUNGLE_STAIRS_NAME,
            CommandBlock => item_id::COMMAND_BLOCK_NAME,
            Beacon => item_id::BEACON_NAME,
            CobblestoneWall => item_id::COBBLESTONE_WALL_NAME,
            FlowerPot => item_id::FLOWER_POT_NAME,
            Carrots => item_id::CARROTS_NAME,
            Potatoes => item_id::POTATOES_NAME,
            WoodenButton => item_id::WOODEN_BUTTON_NAME,
            Skull => item_id::SKULL_NAME,
            Anvil => item_id::ANVIL_NAME,
            TrappedChest => item_id::TRAPPED_CHEST_NAME,
            LightWeightedPressurePlate => item_id::LIGHT_WEIGHTED_PRESSURE_PLATE_NAME,
            HeavyWeightedPressurePlate => item_id::HEAVY_WEIGHTED_PRESSURE_PLATE_NAME,
            UnpoweredComparator => item_id::UNPOWERED_COMPARATOR_NAME,
            PoweredComparator => item_id::POWERED_COMPARATOR_NAME,
            DaylightDetector => item_id::DAYLIGHT_DETECTOR_NAME,
            RedstoneBlock => item_id::REDSTONE_BLOCK_NAME,
            QuartzOre => item_id::QUARTZ_ORE_NAME,
            Hopper => item_id::HOPPER_NAME,
            QuartzBlock => item_id::QUARTZ_BLOCK_NAME,
            QuartzStairs => item_id::QUARTZ_STAIRS_NAME,
            ActivatorRail => item_id::ACTIVATOR_RAIL_NAME,
            Dropper => item_id::DROPPER_NAME,
            StainedHardenedClay => item_id::STAINED_HARDENED_CLAY_NAME,
            StainedGlassPane => item_id::STAINED_GLASS_PANE_NAME,
            Leaves2 => item_id::LEAVES2_NAME,
            Log2 => item_id::LOG2_NAME,
            AcaciaStairs => item_id::ACACIA_STAIRS_NAME,
            DarkOakStairs => item_id::DARK_OAK_STAIRS_NAME,
            Slime => item_id::SLIME_NAME,
            Barrier => item_id::BARRIER_NAME,
            IronTrapdoor => item_id::IRON_TRAPDOOR_NAME,
            Prismarine => item_id::PRISMARINE_NAME,
            SeaLantern => item_id::SEA_LANTERN_NAME,
            HayBlock => item_id::HAY_BLOCK_NAME,
            Carpet => item_id::CARPET_NAME,
            HardenedClay => item_id::HARDENED_CLAY_NAME,
            CoalBlock => item_id::COAL_BLOCK_NAME,
            PackedIce => item_id::PACKED_ICE_NAME,
            DoublePlant => item_id::DOUBLE_PLANT_NAME,
            StandingBanner => item_id::STANDING_BANNER_NAME,
            WallBanner => item_id::WALL_BANNER_NAME,
            DaylightDetectorInverted => item_id::DAYLIGHT_DETECTOR_INVERTED_NAME,
            RedSandstone => item_id::RED_SANDSTONE_NAME,
            RedSandstoneStairs => item_id::RED_SANDSTONE_STAIRS_NAME,
            DoubleStoneSlab2 => item_id::DOUBLE_STONE_SLAB2_NAME,
            StoneSlab2 => item_id::STONE_SLAB2_NAME,
            SpruceFenceGate => item_id::SPRUCE_FENCE_GATE_NAME,
            BirchFenceGate => item_id::BIRCH_FENCE_GATE_NAME,
            JungleFenceGate => item_id::JUNGLE_FENCE_GATE_NAME,
            DarkOakFenceGate => item_id::DARK_OAK_FENCE_GATE_NAME,
            AcaciaFenceGate => item_id::ACACIA_FENCE_GATE_NAME,
            SpruceFence => item_id::SPRUCE_FENCE_NAME,
            BirchFence => item_id::BIRCH_FENCE_NAME,
            JungleFence => item_id::JUNGLE_FENCE_NAME,
            DarkOakFence => item_id::DARK_OAK_FENCE_NAME,
            AcaciaFence => item_id::ACACIA_FENCE_NAME,
            SpruceDoor => item_id::SPRUCE_DOOR_NAME,
            BirchDoor => item_id::BIRCH_DOOR_NAME,
            JungleDoor => item_id::JUNGLE_DOOR_NAME,
            AcaciaDoor => item_id::ACACIA_DOOR_NAME,
            DarkOakDoor => item_id::DARK_OAK_DOOR_NAME,
        }
    }

    pub fn from_name(name: &str) -> Option<Block> {
        use self::Block::*;
        match name {
            item_id::AIR_NAME => Some(Air),
            item_id::STONE_NAME => Some(Stone),
            item_id::GRASS_NAME => Some(Grass),
            item_id::DIRT_NAME => Some(Dirt),
            item_id::COBBLESTONE_NAME => Some(Cobblestone),
            item_id::PLANKS_NAME => Some(Planks),
            item_id::SAPLING_NAME => Some(Sapling),
            item_id::BEDROCK_NAME => Some(Bedrock),
            item_id::FLOWING_WATER_NAME => Some(FlowingWater),
            item_id::WATER_NAME => Some(Water),
            item_id::FLOWING_LAVA_NAME => Some(FlowingLava),
            item_id::LAVA_NAME => Some(Lava),
            item_id::SAND_NAME => Some(Sand),
            item_id::GRAVEL_NAME => Some(Gravel),
            item_id::GOLD_ORE_NAME => Some(GoldOre),
            item_id::IRON_ORE_NAME => Some(IronOre),
            item_id::COAL_ORE_NAME => Some(CoalOre),
            item_id::LOG_NAME => Some(Log),
            item_id::LEAVES_NAME => Some(Leaves),
            item_id::SPONGE_NAME => Some(Sponge),
            item_id::GLASS_NAME => Some(Glass),
            item_id::LAPIS_ORE_NAME => Some(LapisOre),
            item_id::LAPIS_BLOCK_NAME => Some(LapisBlock),
            item_id::DISPENSER_NAME => Some(Dispenser),
            item_id::SANDSTONE_NAME => Some(Sandstone),
            item_id::NOTEBLOCK_NAME => Some(Noteblock),
            item_id::BED_NAME => Some(Bed),
            item_id::GOLDEN_RAIL_NAME => Some(GoldenRail),
            item_id::DETECTOR_RAIL_NAME => Some(DetectorRail),
            item_id::STICKY_PISTON_NAME => Some(StickyPiston),
            item_id::WEB_NAME => Some(Web),
            item_id::TALLGRASS_NAME => Some(Tallgrass),
            item_id::DEADBUSH_NAME => Some(Deadbush),
            item_id::PISTON_NAME => Some(Piston),
            item_id::PISTON_HEAD_NAME => Some(PistonHead),
            item_id::WOOL_NAME => Some(Wool),
            item_id::PISTON_EXTENSION_NAME => Some(PistonExtension),
            item_id::YELLOW_FLOWER_NAME => Some(YellowFlower),
            item_id::RED_FLOWER_NAME => Some(RedFlower),
            item_id::BROWN_MUSHROOM_NAME => Some(BrownMushroom),
            item_id::RED_MUSHROOM_NAME => Some(RedMushroom),
            item_id::GOLD_BLOCK_NAME => Some(GoldBlock),
            item_id::IRON_BLOCK_NAME => Some(IronBlock),
            item_id::DOUBLE_STONE_SLAB_NAME => Some(DoubleStoneSlab),
            item_id::STONE_SLAB_NAME => Some(StoneSlab),
            item_id::BRICK_BLOCK_NAME => Some(BrickBlock),
            item_id::TNT_NAME => Some(Tnt),
            item_id::BOOKSHELF_NAME => Some(Bookshelf),
            item_id::MOSSY_COBBLESTONE_NAME => Some(MossyCobblestone),
            item_id::OBSIDIAN_NAME => Some(Obsidian),
            item_id::TORCH_NAME => Some(Torch),
            item_id::FIRE_NAME => Some(Fire),
            item_id::MOB_SPAWNER_NAME => Some(MobSpawner),
            item_id::OAK_STAIRS_NAME => Some(OakStairs),
            item_id::CHEST_NAME => Some(Chest),
            item_id::REDSTONE_WIRE_NAME => Some(RedstoneWire),
            item_id::DIAMOND_ORE_NAME => Some(DiamondOre),
            item_id::DIAMOND_BLOCK_NAME => Some(DiamondBlock),
            item_id::CRAFTING_TABLE_NAME => Some(CraftingTable),
            item_id::WHEAT_NAME => Some(Wheat),
            item_id::FARMLAND_NAME => Some(Farmland),
            item_id::FURNACE_NAME => Some(Furnace),
            item_id::LIT_FURNACE_NAME => Some(LitFurnace),
            item_id::STANDING_SIGN_NAME => Some(StandingSign),
            item_id::WOODEN_DOOR_NAME => Some(WoodenDoor),
            item_id::LADDER_NAME => Some(Ladder),
            item_id::RAIL_NAME => Some(Rail),
            item_id::STONE_STAIRS_NAME => Some(StoneStairs),
            item_id::WALL_SIGN_NAME => Some(WallSign),
            item_id::LEVER_NAME => Some(Lever),
            item_id::STONE_PRESSURE_PLATE_NAME => Some(StonePressurePlate),
            item_id::IRON_DOOR_NAME => Some(IronDoor),
            item_id::WOODEN_PRESSURE_PLATE_NAME => Some(WoodenPressurePlate),
            item_id::REDSTONE_ORE_NAME => Some(RedstoneOre),
            item_id::LIT_REDSTONE_ORE_NAME => Some(LitRedstoneOre),
            item_id::UNLIT_REDSTONE_TORCH_NAME => Some(UnlitRedstoneTorch),
            item_id::REDSTONE_TORCH_NAME => Some(RedstoneTorch),
            item_id::STONE_BUTTON_NAME => Some(StoneButton),
            item_id::SNOW_LAYER_NAME => Some(SnowLayer),
            item_id::ICE_NAME => Some(Ice),
            item_id::SNOW_NAME => Some(Snow),
            item_id::CACTUS_NAME => Some(Cactus),
            item_id::CLAY_NAME => Some(Clay),
            item_id::REEDS_NAME => Some(Reeds),
            item_id::JUKEBOX_NAME => Some(Jukebox),
            item_id::FENCE_NAME => Some(Fence),
            item_id::PUMPKIN_NAME => Some(Pumpkin),
            item_id::NETHERRACK_NAME => Some(Netherrack),
            item_id::SOUL_SAND_NAME => Some(SoulSand),
            item_id::GLOWSTONE_NAME => Some(Glowstone),
            item_id::PORTAL_NAME => Some(Portal),
            item_id::LIT_PUMPKIN_NAME => Some(LitPumpkin),
            item_id::CAKE_NAME => Some(Cake),
            item_id::UNPOWERED_REPEATER_NAME => Some(UnpoweredRepeater),
            item_id::POWERED_REPEATER_NAME => Some(PoweredRepeater),
            item_id::STAINED_GLASS_NAME => Some(StainedGlass),
            item_id::TRAPDOOR_NAME => Some(Trapdoor),
            item_id::MONSTER_EGG_NAME => Some(MonsterEgg),
            item_id::STONEBRICK_NAME => Some(Stonebrick),
            item_id::BROWN_MUSHROOM_BLOCK_NAME => Some(BrownMushroomBlock),
            item_id::RED_MUSHROOM_BLOCK_NAME => Some(RedMushroomBlock),
            item_id::IRON_BARS_NAME => Some(IronBars),
            item_id::GLASS_PANE_NAME => Some(GlassPane),
            item_id::MELON_BLOCK_NAME => Some(MelonBlock),
            item_id::PUMPKIN_STEM_NAME => Some(PumpkinStem),
            item_id::MELON_STEM_NAME => Some(MelonStem),
            item_id::VINE_NAME => Some(Vine),
            item_id::FENCE_GATE_NAME => Some(FenceGate),
            item_id::BRICK_STAIRS_NAME => Some(BrickStairs),
            item_id::STONE_BRICK_STAIRS_NAME => Some(StoneBrickStairs),
            item_id::MYCELIUM_NAME => Some(Mycelium),
            item_id::WATERLILY_NAME => Some(Waterlily),
            item_id::NETHER_BRICK_NAME => Some(NetherBrick),
            item_id::NETHER_BRICK_FENCE_NAME => Some(NetherBrickFence),
            item_id::NETHER_BRICK_STAIRS_NAME => Some(NetherBrickStairs),
            item_id::NETHER_WART_NAME => Some(NetherWart),
            item_id::ENCHANTING_TABLE_NAME => Some(EnchantingTable),
            item_id::BREWING_STAND_NAME => Some(BrewingStand),
            item_id::CAULDRON_NAME => Some(Cauldron),
            item_id::END_PORTAL_NAME => Some(EndPortal),
            item_id::END_PORTAL_FRAME_NAME => Some(EndPortalFrame),
            item_id::END_STONE_NAME => Some(EndStone),
            item_id::DRAGON_EGG_NAME => Some(DragonEgg),
            item_id::REDSTONE_LAMP_NAME => Some(RedstoneLamp),
            item_id::LIT_REDSTONE_LAMP_NAME => Some(LitRedstoneLamp),
            item_id::DOUBLE_WOODEN_SLAB_NAME => Some(DoubleWoodenSlab),
            item_id::WOODEN_SLAB_NAME => Some(WoodenSlab),
            item_id::COCOA_NAME => Some(Cocoa),
            item_id::SANDSTONE_STAIRS_NAME => Some(SandstoneStairs),
            item_id::EMERALD_ORE_NAME => Some(EmeraldOre),
            item_id::ENDER_CHEST_NAME => Some(EnderChest),
            item_id::TRIPWIRE_HOOK_NAME => Some(TripwireHook),
            item_id::TRIPWIRE_NAME => Some(Tripwire),
            item_id::EMERALD_BLOCK_NAME => Some(EmeraldBlock),
            item_id::SPRUCE_STAIRS_NAME => Some(SpruceStairs),
            item_id::BIRCH_STAIRS_NAME => Some(BirchStairs),
            item_id::JUNGLE_STAIRS_NAME => Some(JungleStairs),
            item_id::COMMAND_BLOCK_NAME => Some(CommandBlock),
            item_id::BEACON_NAME => Some(Beacon),
            item_id::COBBLESTONE_WALL_NAME => Some(CobblestoneWall),
            item_id::FLOWER_POT_NAME => Some(FlowerPot),
            item_id::CARROTS_NAME => Some(Carrots),
            item_id::POTATOES_NAME => Some(Potatoes),
            item_id::WOODEN_BUTTON_NAME => Some(WoodenButton),
            item_id::SKULL_NAME => Some(Skull),
            item_id::ANVIL_NAME => Some(Anvil),
            item_id::TRAPPED_CHEST_NAME => Some(TrappedChest),
            item_id::LIGHT_WEIGHTED_PRESSURE_PLATE_NAME => Some(LightWeightedPressurePlate),
            item_id::HEAVY_WEIGHTED_PRESSURE_PLATE_NAME => Some(HeavyWeightedPressurePlate),
            item_id::UNPOWERED_COMPARATOR_NAME => Some(UnpoweredComparator),
            item_id::POWERED_COMPARATOR_NAME => Some(PoweredComparator),
            item_id::DAYLIGHT_DETECTOR_NAME => Some(DaylightDetector),
            item_id::REDSTONE_BLOCK_NAME => Some(RedstoneBlock),
            item_id::QUARTZ_ORE_NAME => Some(QuartzOre),
            item_id::HOPPER_NAME => Some(Hopper),
            item_id::QUARTZ_BLOCK_NAME => Some(QuartzBlock),
            item_id::QUARTZ_STAIRS_NAME => Some(QuartzStairs),
            item_id::ACTIVATOR_RAIL_NAME => Some(ActivatorRail),
            item_id::DROPPER_NAME => Some(Dropper),
            item_id::STAINED_HARDENED_CLAY_NAME => Some(StainedHardenedClay),
            item_id::STAINED_GLASS_PANE_NAME => Some(StainedGlassPane),
            item_id::LEAVES2_NAME => Some(Leaves2),
            item_id::LOG2_NAME => Some(Log2),
            item_id::ACACIA_STAIRS_NAME => Some(AcaciaStairs),
            item_id::DARK_OAK_STAIRS_NAME => Some(DarkOakStairs),
            item_id::SLIME_NAME => Some(Slime),
            item_id::BARRIER_NAME => Some(Barrier),
            item_id::IRON_TRAPDOOR_NAME => Some(IronTrapdoor),
            item_id::PRISMARINE_NAME => Some(Prismarine),
            item_id::SEA_LANTERN_NAME => Some(SeaLantern),
            item_id::HAY_BLOCK_NAME => Some(HayBlock),
            item_id::CARPET_NAME => Some(Carpet),
            item_id::HARDENED_CLAY_NAME => Some(HardenedClay),
            item_id::COAL_BLOCK_NAME => Some(CoalBlock),
            item_id::PACKED_ICE_NAME => Some(PackedIce),
            item_id::DOUBLE_PLANT_NAME => Some(DoublePlant),
            item_id::STANDING_BANNER_NAME => Some(StandingBanner),
            item_id::WALL_BANNER_NAME => Some(WallBanner),
            item_id::DAYLIGHT_DETECTOR_INVERTED_NAME => Some(DaylightDetectorInverted),
            item_id::RED_SANDSTONE_NAME => Some(RedSandstone),
            item_id::RED_SANDSTONE_STAIRS_NAME => Some(RedSandstoneStairs),
            item_id::DOUBLE_STONE_SLAB2_NAME => Some(DoubleStoneSlab2),
            item_id::STONE_SLAB2_NAME => Some(StoneSlab2),
            item_id::SPRUCE_FENCE_GATE_NAME => Some(SpruceFenceGate),
            item_id::BIRCH_FENCE_GATE_NAME => Some(BirchFenceGate),
            item_id::JUNGLE_FENCE_GATE_NAME => Some(JungleFenceGate),
            item_id::DARK_OAK_FENCE_GATE_NAME => Some(DarkOakFenceGate),
            item_id::ACACIA_FENCE_GATE_NAME => Some(AcaciaFenceGate),
            item_id::SPRUCE_FENCE_NAME => Some(SpruceFence),
            item_id::BIRCH_FENCE_NAME => Some(BirchFence),
            item_id::JUNGLE_FENCE_NAME => Some(JungleFence),
            item_id::DARK_OAK_FENCE_NAME => Some(DarkOakFence),
            item_id::ACACIA_FENCE_NAME => Some(AcaciaFence),
            item_id::SPRUCE_DOOR_NAME => Some(SpruceDoor),
            item_id::BIRCH_DOOR_NAME => Some(BirchDoor),
            item_id::JUNGLE_DOOR_NAME => Some(JungleDoor),
            item_id::ACACIA_DOOR_NAME => Some(AcaciaDoor),
            item_id::DARK_OAK_DOOR_NAME => Some(DarkOakDoor),
            _ => None,
        }
    }

    const STONE_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_STONE, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_STONE, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_STONE, volume: 1.0, pitch: 1.0 },
    };

    const WOOD_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_WOOD, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_WOOD, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_WOOD, volume: 1.0, pitch: 1.0 },
    };

    const GRAVEL_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_GRAVEL, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_GRAVEL, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_GRAVEL, volume: 1.0, pitch: 1.0 },
    };

    const GRASS_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_GRASS, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_GRASS, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_GRASS, volume: 1.0, pitch: 1.0 },
    };

    const METAL_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_STONE, volume: 1.0, pitch: 1.5 },
        step: Sound { name: sound::STEP_STONE, volume: 1.0, pitch: 1.5 },
        place: Sound { name: sound::DIG_STONE, volume: 1.0, pitch: 1.5 },
    };

    const GLASS_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_GLASS, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_STONE, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_STONE, volume: 1.0, pitch: 1.0 },
    };

    const CLOTH_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_CLOTH, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_CLOTH, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_CLOTH, volume: 1.0, pitch: 1.0 },
    };

    const SAND_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_SAND, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_SAND, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_SAND, volume: 1.0, pitch: 1.0 },
    };

    const SNOW_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_SNOW, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_SNOW, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_SNOW, volume: 1.0, pitch: 1.0 },
    };

    const LADDER_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_WOOD, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_LADDER, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::DIG_WOOD, volume: 1.0, pitch: 1.0 },
    };

    const ANVIL_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::DIG_STONE, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::STEP_ANVIL, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::RANDOM_ANVIL_LAND, volume: 1.0, pitch: 1.0 },
    };

    const SLIME_SOUNDS: BlockSounds = BlockSounds {
        breaks: Sound { name: sound::MOB_SLIME_BIG, volume: 1.0, pitch: 1.0 },
        step: Sound { name: sound::MOB_SLIME_SMALL, volume: 1.0, pitch: 1.0 },
        place: Sound { name: sound::MOB_SLIME_BIG, volume: 1.0, pitch: 1.0 },
    };

    pub fn get_sounds(&self) -> Option<BlockSounds> {
        use self::Block::*;
        match *self {
            Air => None,
            Stone => Some(Self::STONE_SOUNDS),
            Grass => Some(Self::GRASS_SOUNDS),
            Dirt => Some(Self::GRAVEL_SOUNDS),
            Cobblestone => Some(Self::STONE_SOUNDS),
            Planks => Some(Self::WOOD_SOUNDS),
            Sapling => Some(Self::GRAVEL_SOUNDS),
            Bedrock => Some(Self::STONE_SOUNDS),
            FlowingWater => None,
            Water => None,
            FlowingLava => None,
            Lava => None,
            Sand => Some(Self::SAND_SOUNDS),
            Gravel => Some(Self::GRAVEL_SOUNDS),
            GoldOre => Some(Self::STONE_SOUNDS),
            IronOre => Some(Self::STONE_SOUNDS),
            CoalOre => Some(Self::STONE_SOUNDS),
            Log => Some(Self::WOOD_SOUNDS),
            Leaves => Some(Self::GRASS_SOUNDS),
            Sponge => Some(Self::GRASS_SOUNDS),
            Glass => Some(Self::GLASS_SOUNDS),
            LapisOre => Some(Self::STONE_SOUNDS),
            LapisBlock => Some(Self::STONE_SOUNDS),
            Dispenser => Some(Self::STONE_SOUNDS),
            Sandstone => Some(Self::STONE_SOUNDS),
            Noteblock => Some(Self::STONE_SOUNDS),
            Bed => Some(Self::WOOD_SOUNDS),
            GoldenRail => Some(Self::METAL_SOUNDS),
            DetectorRail => Some(Self::METAL_SOUNDS),
            StickyPiston => Some(Self::STONE_SOUNDS),
            Web => Some(Self::STONE_SOUNDS),
            Tallgrass => Some(Self::GRASS_SOUNDS),
            Deadbush => Some(Self::GRASS_SOUNDS),
            Piston => Some(Self::STONE_SOUNDS),
            PistonHead => Some(Self::STONE_SOUNDS),
            Wool => Some(Self::CLOTH_SOUNDS),
            PistonExtension => Some(Self::STONE_SOUNDS),
            YellowFlower => Some(Self::GRASS_SOUNDS),
            RedFlower => Some(Self::GRASS_SOUNDS),
            BrownMushroom => Some(Self::GRASS_SOUNDS),
            RedMushroom => Some(Self::GRASS_SOUNDS),
            GoldBlock => Some(Self::STONE_SOUNDS),
            IronBlock => Some(Self::STONE_SOUNDS),
            DoubleStoneSlab => Some(Self::STONE_SOUNDS),
            StoneSlab => Some(Self::STONE_SOUNDS),
            BrickBlock => Some(Self::STONE_SOUNDS),
            Tnt => Some(Self::GRASS_SOUNDS),
            Bookshelf => Some(Self::WOOD_SOUNDS),
            MossyCobblestone => Some(Self::STONE_SOUNDS),
            Obsidian => Some(Self::STONE_SOUNDS),
            Torch => Some(Self::WOOD_SOUNDS),
            Fire => Some(Self::CLOTH_SOUNDS),
            MobSpawner => Some(Self::METAL_SOUNDS),
            OakStairs => Some(Self::WOOD_SOUNDS),
            Chest => Some(Self::WOOD_SOUNDS),
            RedstoneWire => Some(Self::STONE_SOUNDS),
            DiamondOre => Some(Self::STONE_SOUNDS),
            DiamondBlock => Some(Self::STONE_SOUNDS),
            CraftingTable => Some(Self::WOOD_SOUNDS),
            Wheat => Some(Self::GRASS_SOUNDS),
            Farmland => Some(Self::GRAVEL_SOUNDS),
            Furnace => Some(Self::STONE_SOUNDS),
            LitFurnace => Some(Self::STONE_SOUNDS),
            StandingSign => Some(Self::WOOD_SOUNDS),
            WoodenDoor => Some(Self::WOOD_SOUNDS),
            Ladder => Some(Self::LADDER_SOUNDS),
            Rail => Some(Self::METAL_SOUNDS),
            StoneStairs => Some(Self::STONE_SOUNDS),
            WallSign => Some(Self::WOOD_SOUNDS),
            Lever => Some(Self::WOOD_SOUNDS),
            StonePressurePlate => Some(Self::STONE_SOUNDS),
            IronDoor => Some(Self::METAL_SOUNDS),
            WoodenPressurePlate => Some(Self::WOOD_SOUNDS),
            RedstoneOre => Some(Self::STONE_SOUNDS),
            LitRedstoneOre => Some(Self::STONE_SOUNDS),
            UnlitRedstoneTorch => Some(Self::WOOD_SOUNDS),
            RedstoneTorch => Some(Self::WOOD_SOUNDS),
            StoneButton => Some(Self::STONE_SOUNDS),
            SnowLayer => Some(Self::SNOW_SOUNDS),
            Ice => Some(Self::GLASS_SOUNDS),
            Snow => Some(Self::SNOW_SOUNDS),
            Cactus => Some(Self::CLOTH_SOUNDS),
            Clay => Some(Self::GRAVEL_SOUNDS),
            Reeds => Some(Self::GRASS_SOUNDS),
            Jukebox => Some(Self::STONE_SOUNDS),
            Fence => Some(Self::WOOD_SOUNDS),
            Pumpkin => Some(Self::WOOD_SOUNDS),
            Netherrack => Some(Self::STONE_SOUNDS),
            SoulSand => Some(Self::SAND_SOUNDS),
            Glowstone => Some(Self::GLASS_SOUNDS),
            Portal => Some(Self::GLASS_SOUNDS),
            LitPumpkin => Some(Self::WOOD_SOUNDS),
            Cake => Some(Self::CLOTH_SOUNDS),
            UnpoweredRepeater => Some(Self::WOOD_SOUNDS),
            PoweredRepeater => Some(Self::WOOD_SOUNDS),
            StainedGlass => Some(Self::GLASS_SOUNDS),
            Trapdoor => Some(Self::WOOD_SOUNDS),
            MonsterEgg => Some(Self::STONE_SOUNDS),
            Stonebrick => Some(Self::STONE_SOUNDS),
            BrownMushroomBlock => Some(Self::WOOD_SOUNDS),
            RedMushroomBlock => Some(Self::WOOD_SOUNDS),
            IronBars => Some(Self::METAL_SOUNDS),
            GlassPane => Some(Self::GLASS_SOUNDS),
            MelonBlock => Some(Self::WOOD_SOUNDS),
            PumpkinStem => Some(Self::WOOD_SOUNDS),
            MelonStem => Some(Self::WOOD_SOUNDS),
            Vine => Some(Self::GRASS_SOUNDS),
            FenceGate => Some(Self::WOOD_SOUNDS),
            BrickStairs => Some(Self::STONE_SOUNDS),
            StoneBrickStairs => Some(Self::STONE_SOUNDS),
            Mycelium => Some(Self::GRASS_SOUNDS),
            Waterlily => Some(Self::GRASS_SOUNDS),
            NetherBrick => Some(Self::STONE_SOUNDS),
            NetherBrickFence => Some(Self::STONE_SOUNDS),
            NetherBrickStairs => Some(Self::STONE_SOUNDS),
            NetherWart => None,
            EnchantingTable => Some(Self::STONE_SOUNDS),
            BrewingStand => Some(Self::STONE_SOUNDS),
            Cauldron => Some(Self::STONE_SOUNDS),
            EndPortal => Some(Self::GLASS_SOUNDS),
            EndPortalFrame => Some(Self::GLASS_SOUNDS),
            EndStone => Some(Self::STONE_SOUNDS),
            DragonEgg => Some(Self::STONE_SOUNDS),
            RedstoneLamp => Some(Self::GLASS_SOUNDS),
            LitRedstoneLamp => Some(Self::GLASS_SOUNDS),
            DoubleWoodenSlab => Some(Self::WOOD_SOUNDS),
            WoodenSlab => Some(Self::WOOD_SOUNDS),
            Cocoa => Some(Self::WOOD_SOUNDS),
            SandstoneStairs => Some(Self::STONE_SOUNDS),
            EmeraldOre => Some(Self::STONE_SOUNDS),
            EnderChest => Some(Self::STONE_SOUNDS),
            TripwireHook => Some(Self::STONE_SOUNDS),
            Tripwire => None,
            EmeraldBlock => Some(Self::STONE_SOUNDS),
            SpruceStairs => Some(Self::WOOD_SOUNDS),
            BirchStairs => Some(Self::WOOD_SOUNDS),
            JungleStairs => Some(Self::WOOD_SOUNDS),
            CommandBlock => Some(Self::WOOD_SOUNDS),
            Beacon => Some(Self::STONE_SOUNDS),
            CobblestoneWall => Some(Self::STONE_SOUNDS),
            FlowerPot => Some(Self::STONE_SOUNDS),
            Carrots => Some(Self::GRASS_SOUNDS),
            Potatoes => Some(Self::GRASS_SOUNDS),
            WoodenButton => Some(Self::WOOD_SOUNDS),
            Skull => Some(Self::STONE_SOUNDS),
            Anvil => Some(Self::ANVIL_SOUNDS),
            TrappedChest => Some(Self::WOOD_SOUNDS),
            LightWeightedPressurePlate => Some(Self::WOOD_SOUNDS),
            HeavyWeightedPressurePlate => Some(Self::WOOD_SOUNDS),
            UnpoweredComparator => Some(Self::WOOD_SOUNDS),
            PoweredComparator => Some(Self::WOOD_SOUNDS),
            DaylightDetector => Some(Self::WOOD_SOUNDS),
            RedstoneBlock => Some(Self::STONE_SOUNDS),
            QuartzOre => Some(Self::STONE_SOUNDS),
            Hopper => Some(Self::STONE_SOUNDS),
            QuartzBlock => Some(Self::STONE_SOUNDS),
            QuartzStairs => Some(Self::STONE_SOUNDS),
            ActivatorRail => Some(Self::METAL_SOUNDS),
            Dropper => Some(Self::STONE_SOUNDS),
            StainedHardenedClay => Some(Self::STONE_SOUNDS),
            StainedGlassPane => Some(Self::GLASS_SOUNDS),
            Leaves2 => Some(Self::GRASS_SOUNDS),
            Log2 => Some(Self::WOOD_SOUNDS),
            AcaciaStairs => Some(Self::WOOD_SOUNDS),
            DarkOakStairs => Some(Self::WOOD_SOUNDS),
            Slime => Some(Self::SLIME_SOUNDS),
            Barrier => Some(Self::STONE_SOUNDS),
            IronTrapdoor => Some(Self::METAL_SOUNDS),
            Prismarine => Some(Self::STONE_SOUNDS),
            SeaLantern => Some(Self::GLASS_SOUNDS),
            HayBlock => Some(Self::GRASS_SOUNDS),
            Carpet => Some(Self::CLOTH_SOUNDS),
            HardenedClay => Some(Self::STONE_SOUNDS),
            CoalBlock => Some(Self::STONE_SOUNDS),
            PackedIce => Some(Self::GLASS_SOUNDS),
            DoublePlant => Some(Self::GRASS_SOUNDS),
            StandingBanner => Some(Self::WOOD_SOUNDS),
            WallBanner => Some(Self::WOOD_SOUNDS),
            DaylightDetectorInverted => Some(Self::WOOD_SOUNDS),
            RedSandstone => Some(Self::STONE_SOUNDS),
            RedSandstoneStairs => Some(Self::STONE_SOUNDS),
            DoubleStoneSlab2 => Some(Self::STONE_SOUNDS),
            StoneSlab2 => Some(Self::STONE_SOUNDS),
            SpruceFenceGate => Some(Self::WOOD_SOUNDS),
            BirchFenceGate => Some(Self::WOOD_SOUNDS),
            JungleFenceGate => Some(Self::WOOD_SOUNDS),
            DarkOakFenceGate => Some(Self::WOOD_SOUNDS),
            AcaciaFenceGate => Some(Self::WOOD_SOUNDS),
            SpruceFence => Some(Self::WOOD_SOUNDS),
            BirchFence => Some(Self::WOOD_SOUNDS),
            JungleFence => Some(Self::WOOD_SOUNDS),
            DarkOakFence => Some(Self::WOOD_SOUNDS),
            AcaciaFence => Some(Self::WOOD_SOUNDS),
            SpruceDoor => Some(Self::WOOD_SOUNDS),
            BirchDoor => Some(Self::WOOD_SOUNDS),
            JungleDoor => Some(Self::WOOD_SOUNDS),
            AcaciaDoor => Some(Self::WOOD_SOUNDS),
            DarkOakDoor => Some(Self::WOOD_SOUNDS),
        }
    }

    pub fn create_new_block_entity(&self) -> Option<Box<BlockEntity>> {
        use self::Block::*;
        match *self {
            Furnace => Some(Box::new(block_entity::Furnace::default())),
            Chest => Some(Box::new(block_entity::Chest::default())),
            EnderChest => Some(Box::new(block_entity::EnderChest::default())),
            Jukebox => Some(Box::new(block_entity::Jukebox::default())),
            Dispenser => Some(Box::new(block_entity::Dispenser::default())),
            Dropper => Some(Box::new(block_entity::Dropper::default())),
            StandingSign => Some(Box::new(block_entity::Sign::default())),
            WallSign => Some(Box::new(block_entity::Sign::default())),
            MobSpawner => Some(Box::new(block_entity::MobSpawner::default())),
            Noteblock => Some(Box::new(block_entity::Noteblock::default())),
            Piston => Some(Box::new(block_entity::Piston::default())),
            BrewingStand => Some(Box::new(block_entity::BrewingStand::default())),
            EnchantingTable => Some(Box::new(block_entity::EnchantmentTable::default())),
            EndPortal => Some(Box::new(block_entity::EndPortal::default())),
            CommandBlock => Some(Box::new(block_entity::CommandBlock::default())),
            Beacon => Some(Box::new(block_entity::Beacon::default())),
            Skull => Some(Box::new(block_entity::Skull::default())),
            DaylightDetector => Some(Box::new(block_entity::DaylightDetector::default())),
            Hopper => Some(Box::new(block_entity::Hopper::default())),
            PoweredComparator => Some(Box::new(block_entity::Comparator::default())),
            UnpoweredComparator => Some(Box::new(block_entity::Comparator::default())),
            FlowerPot => Some(Box::new(block_entity::FlowerPot::default())),
            StandingBanner => Some(Box::new(block_entity::Banner::default())),
            WallBanner => Some(Box::new(block_entity::Banner::default())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BlockSounds {
    pub breaks: Sound,
    pub step: Sound,
    pub place: Sound,
}