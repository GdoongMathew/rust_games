use crate::game::stat::{Stat, StatTrait};

pub trait Profession {
    fn profession_type(&self) -> ProfessionType;
    fn effective_against<P: Profession>(&self, profession: &P) -> bool;
    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool;

    fn attack_points<T: StatTrait>(&self, stat_trait: &T) -> i32;
    fn defense_points<T: StatTrait>(&self, stat_trait: &T) -> i32;
}

pub struct Warrior {
    stat: Stat,
}

impl Warrior {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 90,
                attack: 40,
                defense: 55,
                magic: 0,
            },
        }
    }
}


impl Profession for Warrior {
    fn profession_type(&self) -> ProfessionType {
        ProfessionType::WarriorType
    }
    fn effective_against<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::KnightType)
    }

    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::SorcererType)
    }

    fn attack_points<T: StatTrait>(&self, stat_trait: &T) -> i32 {
        stat_trait.get_stat().attack
    }

    fn defense_points<T: StatTrait>(&self, stat_trait: &T) -> i32 {
        stat_trait.get_stat().defense
    }
}

pub struct Sorcerer {
    stat: Stat,
}

impl Sorcerer {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 70,
                attack: 0,
                defense: 20,
                magic: 50,
            },
        }
    }
}

impl Profession for Sorcerer {
    fn profession_type(&self) -> ProfessionType {
        ProfessionType::SorcererType
    }

    fn effective_against<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::WarriorType)
    }

    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::KnightType)
    }
    fn attack_points<T: StatTrait>(&self, stat_trait: &T) -> i32 {
        stat_trait.get_stat().magic
    }
    fn defense_points<T: StatTrait>(&self, stat_trait: &T) -> i32 {
        stat_trait.get_stat().defense
    }
}


pub struct Knight {
    stat: Stat,
}

impl Knight {
    pub fn new() -> Self {
        Self {
            stat: Stat {
                health: 100,
                attack: 40,
                defense: 30,
                magic: 0,
            },
        }
    }
}

impl Profession for Knight {
    fn profession_type(&self) -> ProfessionType {
        ProfessionType::KnightType
    }
    fn effective_against<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::SorcererType)
    }

    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::WarriorType)
    }
    fn attack_points<T: StatTrait>(&self, stat_trait: &T) -> i32 {
        stat_trait.get_stat().attack
    }
    fn defense_points<T: StatTrait>(&self, stat_trait: &T) -> i32 {
        stat_trait.get_stat().defense
    }
}


#[derive(Debug, PartialEq)]
enum ProfessionType {
    WarriorType,
    SorcererType,
    KnightType,
}


#[cfg(test)]
mod profession_tests {
    use super::*;

    #[test]
    fn test_new_warrior_stat() {
        let warrior = Warrior::new();
        assert_eq!(warrior.stat, Stat { health: 90, attack: 40, defense: 55, magic: 0 });
    }

    #[test]
    fn test_new_sorcerer_stat() {
        let sorcerer = Sorcerer::new();
        assert_eq!(sorcerer.stat, Stat { health: 70, attack: 0, defense: 20, magic: 50 });
    }

    #[test]
    fn test_new_knight_stat() {
        let knight = Knight::new();
        assert_eq!(knight.stat, Stat { health: 100, attack: 40, defense: 30, magic: 0 });
    }

    #[test]
    fn test_warrior_profession_type() {
        let warrior = Warrior::new();
        assert_eq!(warrior.profession_type(), ProfessionType::WarriorType);
    }

    #[test]
    fn test_sorcerer_profession_type() {
        let sorcerer = Sorcerer::new();
        assert_eq!(sorcerer.profession_type(), ProfessionType::SorcererType);
    }

    #[test]
    fn test_knight_profession_type() {
        let knight = Knight::new();
        assert_eq!(knight.profession_type(), ProfessionType::KnightType);
    }

    #[test]
    fn test_warrior_effective_against_type() {
        let warrior = Warrior::new();
        let knight = Knight::new();
        let sorcerer = Sorcerer::new();
        assert_eq!(warrior.effective_against(&knight), true);
        assert_eq!(warrior.effective_against(&sorcerer), false);
    }

    #[test]
    fn test_warrior_suppressed_by_type() {
        let warrior = Warrior::new();
        let knight = Knight::new();
        let sorcerer = Sorcerer::new();
        assert_eq!(warrior.suppressed_by(&knight), false);
        assert_eq!(warrior.suppressed_by(&sorcerer), true);
    }

    #[test]
    fn test_sorcerer_effective_against_type() {
        let warrior = Warrior::new();
        let knight = Knight::new();
        let sorcerer = Sorcerer::new();
        assert_eq!(sorcerer.effective_against(&warrior), true);
        assert_eq!(sorcerer.effective_against(&knight), false);
    }

    #[test]
    fn test_sorcerer_suppressed_by_type() {
        let warrior = Warrior::new();
        let knight = Knight::new();
        let sorcerer = Sorcerer::new();
        assert_eq!(sorcerer.suppressed_by(&knight), true);
        assert_eq!(sorcerer.suppressed_by(&warrior), false);
    }

    #[test]
    fn test_knight_effective_against_type() {
        let warrior = Warrior::new();
        let knight = Knight::new();
        let sorcerer = Sorcerer::new();
        assert_eq!(knight.effective_against(&sorcerer), true);
        assert_eq!(knight.effective_against(&warrior), false);
    }

    #[test]
    fn test_knight_suppressed_by_type() {
        let warrior = Warrior::new();
        let knight = Knight::new();
        let sorcerer = Sorcerer::new();
        assert_eq!(knight.suppressed_by(&sorcerer), false);
        assert_eq!(knight.suppressed_by(&warrior), true);
    }

    struct TestProp;

    impl StatTrait for TestProp {
        fn get_stat(&self) -> Stat {
            Stat{health: 10, attack: 11, defense: 12, magic: 13}
        }

        fn set_stat(&self, stat: Stat) -> () {
            todo!()
        }
    }

    #[test]
    fn test_sorcerer_attack_points() {
        let prop = TestProp{};
        let prof = Sorcerer::new();
        assert_eq!(prof.attack_points(&prop), prop.get_stat().magic)
    }

    #[test]
    fn test_sorcerer_defense_points() {
        let prop = TestProp{};
        let prof = Sorcerer::new();
        assert_eq!(prof.defense_points(&prop), prop.get_stat().defense)
    }

    #[test]
    fn test_knight_attack_points() {
        let prop = TestProp{};
        let prof = Knight::new();
        assert_eq!(prof.attack_points(&prop), prop.get_stat().attack)
    }

    #[test]
    fn test_knight_defense_points() {
        let prop = TestProp{};
        let prof = Knight::new();
        assert_eq!(prof.defense_points(&prop), prop.get_stat().defense)
    }

    #[test]
    fn test_warrior_attack_points() {
        let prop = TestProp{};
        let prof = Warrior::new();
        assert_eq!(prof.attack_points(&prop), prop.get_stat().attack)
    }

    #[test]
    fn test_warrior_defense_points() {
        let prop = TestProp{};
        let prof = Warrior::new();
        assert_eq!(prof.defense_points(&prop), prop.get_stat().defense)
    }
}
