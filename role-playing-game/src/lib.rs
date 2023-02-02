// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {

    fn new(mana: Option<u32>, level: u32) -> Player {

        Self {
            health: 100,
            mana: mana,
            level: level,
        }

    }

    pub fn revive(&self) -> Option<Player> {

        if self.health != 0 {
            return None
        }

        let new_mana = if self.level >= 10 { Some(100) } else { None };
        return Some(Player::new(new_mana, self.level))
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {

        match self.mana {
            None => {
                self.health = self.health - min(self.health, mana_cost);
                return 0
            },
            Some(m) => {
                if m >= mana_cost {
                    self.mana = Some(m - mana_cost);
                    return mana_cost * 2
                }
                else {
                    return 0
                }
            }
        }

    }
}
