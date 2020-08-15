use item::Item;

pub struct Inventory {
    head: Option<Item>,
    chest: Option<Item>,
    legs: Option<Item>,
    feet: Option<Item>,
    crafting: [Option<Item>; 4],
    crafting_result: Option<Item>,
    main: [Option<Item>; 36],
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            head: None,
            chest: None,
            legs: None,
            feet: None,
            crafting: [None; 4],
            crafting_result: None,
            main: [None; 36],
        }
    }
}