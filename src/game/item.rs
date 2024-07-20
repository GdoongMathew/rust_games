use crate::game::stat::{Stat, StatTrait};

pub trait ItemTrait {
    fn create_effect<T: StatTrait>(&self, target: &T) -> ();
}

pub struct Helmet {
    stat: Stat,
}

impl Helmet {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 0,
                attack: 0,
                defense: 10,
                magic: 0,
            },
        }
    }
}

impl ItemTrait for Helmet {
    fn create_effect<T: StatTrait>(&self, target: &T) -> () {
        target.set_stat(self.stat + target.get_stat());
    }
}

pub struct ChestPlate {
    stat: Stat,
}

impl ChestPlate {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 0,
                attack: 0,
                defense: 15,
                magic: 0,
            },
        }
    }
}

impl ItemTrait for ChestPlate {
    fn create_effect<T: StatTrait>(&self, target: &T) -> () {
        target.set_stat(self.stat + target.get_stat());
    }
}

pub struct Leggings {
    stat: Stat,
}

impl Leggings {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 0,
                attack: 0,
                defense: 10,
                magic: 0,
            },
        }
    }
}

impl ItemTrait for Leggings {
    fn create_effect<T: StatTrait>(&self, target: &T) -> () {
        target.set_stat(self.stat + target.get_stat());
    }
}

pub struct Sword {
    stat: Stat,
}

impl Sword {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 0,
                attack: 15,
                defense: 4,
                magic: 0,
            },
        }
    }
}

impl ItemTrait for Sword {
    fn create_effect<T: StatTrait>(&self, target: &T) -> () {
        let stat = Stat {
            health: 0,
            attack: 15,
            defense: 4,
            magic: 0,
        };
        target.set_stat(stat + target.get_stat());
    }
}

pub struct BloodBag {
    stat: Stat,
}

impl BloodBag {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 30,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        }
    }
}

impl ItemTrait for BloodBag {
    fn create_effect<T: StatTrait>(&self, target: &T) -> () {
        target.set_stat(self.stat + target.get_stat());
    }
}

pub struct Wand {
    stat: Stat,
}

impl Wand {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 70,
            },
        }
    }
}

impl ItemTrait for Wand {
    fn create_effect<T: StatTrait>(&self, target: &T) -> () {
        let stat = Stat {
            health: 0,
            attack: 0,
            defense: 0,
            magic: 70,
        };
        target.set_stat(stat + target.get_stat());
    }
}

#[cfg(test)]
mod test_item {

    use super::*;

    #[test]
    fn test_new_helmet() {
        let helmet = Helmet::new();
        assert_eq!(
            helmet.stat,
            Stat {
                health: 0,
                attack: 0,
                defense: 10,
                magic: 0
            }
        );
    }

    #[test]
    fn test_new_chestplate() {
        let chestplate = ChestPlate::new();
        assert_eq!(
            chestplate.stat,
            Stat {
                health: 0,
                attack: 0,
                defense: 15,
                magic: 0
            }
        );
    }

    #[test]
    fn test_new_leggings() {
        let leggings = Leggings::new();
        assert_eq!(
            leggings.stat,
            Stat {
                health: 0,
                attack: 0,
                defense: 10,
                magic: 0
            }
        );
    }

    #[test]
    fn test_new_sword() {
        let sword = Sword::new();
        assert_eq!(
            sword.stat,
            Stat {
                health: 0,
                attack: 15,
                defense: 4,
                magic: 0
            }
        );
    }

    #[test]
    fn test_new_blood_bag() {
        let blood_bag = BloodBag::new();
        assert_eq!(
            blood_bag.stat,
            Stat {
                health: 30,
                attack: 0,
                defense: 0,
                magic: 0
            }
        );
    }

    #[test]
    fn test_new_wand() {
        let wand = Wand::new();
        assert_eq!(
            wand.stat,
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 70
            }
        );
    }

    use std::cell::Cell;

    struct MockStat {
        stat: Cell<Stat>,
    }

    impl MockStat {
        fn new(stat: Stat) -> Self {
            Self {
                stat: Cell::new(stat),
            }
        }
    }

    impl StatTrait for MockStat {
        fn get_stat(&self) -> Stat {
            self.stat.get()
        }

        fn set_stat(&self, stat: Stat) -> () {
            self.stat.set(stat);
        }
    }

    #[test]
    fn test_helmet_create_effect() {
        let helmet = Helmet::new();
        let mut mock_stat = MockStat::new(
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        );
        helmet.create_effect(&mut mock_stat);
        assert_eq!(
            mock_stat.get_stat(),
            Stat {
                health: 0,
                attack: 0,
                defense: 10,
                magic: 0
            }
        );
    }

    #[test]
    fn test_chestplate_create_effect() {
        let chestplate = ChestPlate::new();
        let mut mock_stat = MockStat::new(
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        );
        chestplate.create_effect(&mut mock_stat);
        assert_eq!(
            mock_stat.get_stat(),
            Stat {
                health: 0,
                attack: 0,
                defense: 15,
                magic: 0
            }
        );
    }

    #[test]
    fn test_leggings_create_effect() {
        let leggings = Leggings::new();
        let mut mock_stat = MockStat::new(
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        );
        leggings.create_effect(&mut mock_stat);
        assert_eq!(
            mock_stat.get_stat(),
            Stat {
                health: 0,
                attack: 0,
                defense: 10,
                magic: 0
            }
        );
    }

    #[test]
    fn test_sword_create_effect() {
        let sword = Sword::new();
        let mut mock_stat = MockStat::new(
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        );
        sword.create_effect(&mut mock_stat);
        assert_eq!(
            mock_stat.get_stat(),
            Stat {
                health: 0,
                attack: 15,
                defense: 4,
                magic: 0
            }
        );
    }

    #[test]
    fn test_blood_bag_create_effect() {
        let blood_bag = BloodBag::new();
        let mut mock_stat = MockStat::new(
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        );
        blood_bag.create_effect(&mut mock_stat);
        assert_eq!(
            mock_stat.get_stat(),
            Stat {
                health: 30,
                attack: 0,
                defense: 0,
                magic: 0
            }
        );
    }

    #[test]
    fn test_wand_create_effect() {
        let wand = Wand::new();
        let mut mock_stat = MockStat::new(
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 0,
            },
        );
        wand.create_effect(&mut mock_stat);
        assert_eq!(
            mock_stat.get_stat(),
            Stat {
                health: 0,
                attack: 0,
                defense: 0,
                magic: 70
            }
        );
    }

}
