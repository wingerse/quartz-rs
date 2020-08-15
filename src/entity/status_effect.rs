#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    Speed = 1,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
}

impl StatusEffect {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(x: i32) -> Option<StatusEffect> {
        use self::StatusEffect::*;
        match x {
            1 => Some(Speed),
            2 => Some(Slowness),
            3 => Some(Haste),
            4 => Some(MiningFatigue),
            5 => Some(Strength),
            6 => Some(InstantHealth),
            7 => Some(InstantDamage),
            8 => Some(JumpBoost),
            9 => Some(Nausea),
            10 => Some(Regeneration),
            11 => Some(Resistance),
            12 => Some(FireResistance),
            13 => Some(WaterBreathing),
            14 => Some(Invisibility),
            15 => Some(Blindness),
            16 => Some(NightVision),
            17 => Some(Hunger),
            18 => Some(Weakness),
            19 => Some(Poison),
            20 => Some(Wither),
            21 => Some(HealthBoost),
            22 => Some(Absorption),
            23 => Some(Saturation),
            _ => None,
        }
    }
}