// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mana = if self.level < 100 {
                Some(100_u32)
            } else {
                None
            };

            return Some(Player {
                health: 100,
                mana,
                level: self.level,
            });
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana < mana_cost {
                return 0;
            } else {
                let val = mana - mana_cost;
                self.mana = Some(val);
                return mana_cost * 2;
            }
        }

        if self.health < mana_cost {
            self.health = 0;
            self.revive();
        } else {
            self.health -= mana_cost;
        }

        0
    }
}
