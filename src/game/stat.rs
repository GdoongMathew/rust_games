use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Stat {
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
}

pub trait StatTrait {
    fn get_stat(&self) -> Stat;
    fn set_stat(&self, stat: Stat) -> ();
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "State: [health: {health}, attack: {attack}, defense: {defense}, magic: {magic}]",
            health = self.health,
            attack = self.attack,
            defense = self.defense,
            magic = self.magic
        )
    }
}

impl Add for Stat {
    type Output = Self;

    /// Add two stat member to a new `Stat`
    /// # Examples
    /// ```
    /// use game::Stat
    /// let stat = Stat {health: 10, attack: 10, defense: 10, magic: 10};
    /// let stat2 = Stat {health: 10, attack: 10, defense: 10, magic: 10};
    /// let stat3 = stat + stat2;
    /// assert_eq!(stat3, Stat {health: 20, attack: 20, defense: 20, magic: 20});
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            health: self.health + rhs.health,
            attack: self.attack + rhs.attack,
            defense: self.defense + rhs.defense,
            magic: self.magic + rhs.magic,
        }
    }
}

impl AddAssign for Stat {
    fn add_assign(&mut self, rhs: Self) {
        self.health += rhs.health;
        self.attack += rhs.attack;
        self.defense += rhs.defense;
        self.magic += rhs.magic;
    }
}

impl Sub for Stat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            health: self.health - rhs.health,
            attack: self.attack - rhs.attack,
            defense: self.defense - rhs.defense,
            magic: self.magic - rhs.magic,
        }
    }
}

impl SubAssign for Stat {
    fn sub_assign(&mut self, rhs: Self) {
        self.health -= rhs.health;
        self.attack -= rhs.attack;
        self.defense -= rhs.defense;
        self.magic -= rhs.magic;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::ide;
    use parameterized::parameterized as pm;

    #[test]
    fn test_stat_display(){
        let stat = Stat {
            health: 100,
            attack: 100,
            magic: 100,
            defense: 100
        };

        assert_eq!(format!("{}", stat), "State: [health: 100, attack: 100, defense: 100, magic: 100]\n");
    }

    mod stat_tests {
        use super::*;
        ide!();

        #[pm(stat = {
            Stat {health: 10, attack: 10, defense: 10, magic: 10},
            Stat {health: 0, attack: 0, defense: 0, magic: 0},
            Stat {health: -10, attack: -10, defense: -10, magic: -10},
        }, expect = {
            Stat {health: 0, attack: 10, defense: 110, magic: 29},
            Stat {health: -10, attack: 0, defense: 100, magic: 19},
            Stat {health: -20, attack: -10, defense: 90, magic: 9},
        })]
        fn test_stat_add(stat: Stat, expect: Stat) {
            let stat2 = Stat {
                health: -10,
                attack: 0,
                defense: 100,
                magic: 19,
            };
            assert_eq!(stat + stat2, expect)
        }

        #[pm(stat = {
            Stat {health: 10, attack: 10, defense: 10, magic: 10},
            Stat {health: 0, attack: 0, defense: 0, magic: 0},
            Stat {health: -10, attack: -10, defense: -10, magic: -10},
        }, expect = {
            Stat {health: 20, attack: 10, defense: -90, magic: -9},
            Stat {health: 10, attack: 0, defense: -100, magic: -19},
            Stat {health: 0, attack: -10, defense: -110, magic: -29},
        })]
        fn test_stat_sub(stat: Stat, expect: Stat) {
            let stat2 = Stat {
                health: -10,
                attack: 0,
                defense: 100,
                magic: 19,
            };
            assert_eq!(stat - stat2, expect)
        }

        #[test]
        fn test_stat_add_assign() {
            let mut stat = Stat {
                health: -10,
                attack: -10,
                defense: -10,
                magic: -10,
            };

            let stat2 = Stat {
                health: 10,
                attack: 10,
                defense: 10,
                magic: 10,
            };

            stat += stat2;
            assert_eq!(stat, Stat{health: 0, attack: 0, defense: 0, magic: 0})
        }

        #[test]
        fn test_stat_sub_assign(){
            let mut stat = Stat {
                health: 10,
                attack: 10,
                defense: 10,
                magic: 10,
            };
            let stat2 = Stat {
                health: 10,
                attack: 10,
                defense: 10,
                magic: 10,
            };
            stat -= stat2;
            assert_eq!(stat, Stat{health: 0, attack: 0, defense: 0, magic: 0})
        }
    }
}
