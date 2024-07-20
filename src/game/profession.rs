use crate::game::stat::Stat;

pub trait Profession {
    fn profession_type(&self) -> ProfessionType;
    fn effective_against<P: Profession>(&self, profession: &P) -> bool;
    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool;
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
        ProfessionType::Warrior
    }
    fn effective_against<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::Knight)
    }

    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::Sorcerer)
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
        ProfessionType::Sorcerer
    }

    fn effective_against<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::Warrior)
    }

    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::Knight)
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
        ProfessionType::Knight
    }
    fn effective_against<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::Sorcerer)
    }

    fn suppressed_by<P: Profession>(&self, profession: &P) -> bool {
        matches! (profession.profession_type(), ProfessionType::Warrior)
    }
}


#[derive(Debug, PartialEq)]
enum ProfessionType {
    Warrior,
    Sorcerer,
    Knight,
}


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
        assert_eq!(warrior.profession_type(), ProfessionType::Warrior);
    }

    #[test]
    fn test_sorcerer_profession_type() {
        let sorcerer = Sorcerer::new();
        assert_eq!(sorcerer.profession_type(), ProfessionType::Sorcerer);
    }

    #[test]
    fn test_knight_profession_type() {
        let knight = Knight::new();
        assert_eq!(knight.profession_type(), ProfessionType::Knight);
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
}
