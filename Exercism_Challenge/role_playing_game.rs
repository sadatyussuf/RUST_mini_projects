pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}
impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            let newPlayer = Player {
                health : 100,
                mana : (self.level > 9).then(|| 100),
                level : self.level
            };
            return Some(newPlayer)
        }
        else {
            return None;
        }
    }
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // match self.mana {
        //     None => {
        //         // self.health -= mana_cost;
        //         self.health = self.health.saturating_sub(mana_cost);
        //         return 0
        //     }
        //     Some(mana) if mana_cost > mana => 0,
        //     Some(mana)  =>{
        //         self.mana = Some(mana.saturating_sub(mana_cost));
        //         return mana_cost * 2
        //     },
            
        // }
        if self.mana.is_none(){
            self.health = self.health.saturating_sub(mana_cost); 
            return 0   
        }
        else if  Some(mana_cost) > self.mana{
                return 0
        }
        else{      
            self.mana = Some(self.mana.unwrap().saturating_sub(mana_cost));
            return mana_cost * 2
        }
    }
}