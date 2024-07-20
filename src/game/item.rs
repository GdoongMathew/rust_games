use crate::game::stat::{Stat, StatTrait};

pub trait ItemTrait {
    fn create_effect<T: StatTrait>(&self, target: &T) -> ();
}

pub struct Helmet {
    stat: Stat
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
    fn create_effect<T: StatTrait>(&self, target: &T) -> (){
        target.set_stat(self.stat + target.get_stat());
    }
}

pub struct ChestPlate {
    stat: Stat
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
    stat: Stat
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
    stat: Stat
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
    stat: Stat
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
    stat: Stat
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

