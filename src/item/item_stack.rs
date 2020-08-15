use std::collections::HashMap;

use item::Item;
use nbt::{Compound, Nbt};
use proto::data::SlotData;

#[derive(Debug)]
pub struct ItemStack {
    item: Box<Item>,
    count: i8,
}

impl ItemStack {
    pub fn new(item: Box<Item>, count: i8) -> ItemStack {
        ItemStack { item, count }
    }

    fn to_nbt_impl(&self, slot: Option<u8>) -> Compound {
        let mut compound = Compound(HashMap::new());
        compound.0.insert("Count".into(), self.count.into());
        if let Some(slot) = slot {
            compound.0.insert("Slot".into(), (slot as i8).into());
        }
        compound.0.insert("Damage".into(), self.item.get_damage_value().into());
        compound.0.insert("id".into(), self.item.get_name().to_string().into());
        let mut tag = Compound(HashMap::new());
        self.item.update_tag(&mut tag);
        if !tag.0.is_empty() {
            compound.0.insert("tag".into(), tag.into());
        }

        compound
    }

    pub fn to_slot_data(&self) -> SlotData {
        SlotData::Some {
            id: self.item.get_id() as i16,
            item_count: self.count,
            item_damage: self.item.get_damage_value(),
            tag: {
                let mut tag = Compound(HashMap::new());
                self.item.update_tag(&mut tag);
                if tag.0.is_empty() { Nbt::Empty } else { Nbt::Some("tag".into(), tag) }
            },
        }
    }

    pub fn to_nbt_with_slot(&self, slot: u8) -> Compound {
        self.to_nbt_impl(Some(slot))
    }

    pub fn to_nbt(&self) -> Compound {
        self.to_nbt_impl(None)
    }
}
