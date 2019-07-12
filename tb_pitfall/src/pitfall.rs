use toybox_core;
use toybox_core::random;
use types::*;

use serde_json;

use toybox_core::graphics::Drawable;
use toybox_core::Input;
use toybox_core::QueryError;

mod screen { 
   pub const SEAFOAM: (u8, u8, u8) = (1, 2, 3); //TODO : Numbers
   pub const GROUND_OFFSET: u8 = 14; 
}

impl Pitfall { 
}

impl Default for Pitfall { 
    fn default() -> Self {
       Pitfall {
          // instantiate all of the stuff
      }
    }
}

impl toybox_core::Simulation for Pitfall { 
    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<toybox_core::State>, serde_json::Error> {
        let state: StateCore = serde_json::from_str(json_str)?;
        Ok(Box::new(State {
            config: self.clone(),
            state,
        }))
    }

    fn from_json(&self, json_str: &str) -> Result<Box<toybox_core::Simulation>, serde_json::Error> {
        let config: Pitfall = serde_json::from_str(json_str)?;
        Ok(Box::new(config))
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Pitfall should be JSON-serializable!")
    }

    fn reset_seed(&mut self, seed:u32){

    }

    fn new_game(&mut self) -> Box<toybox_core::State> {
        Box::new(State{
            state : StateCore::from_config(&self),
            config: self.clone(),
        })
    }

    fn game_size(&self) -> (i32,i32) {
        (100,100)
    }

    fn legal_action_set(&self) -> Vec<toybox_core::AleAction>{
        Vec::new()
    }

}

impl State { 
} 

impl toybox_core::State for State { 
    fn lives(&self) -> i32{
        1
    }

    fn score(&self) -> i32{
        42
    }

    fn update_mut(&mut self, buttons: Input) {

    }

    fn draw(&self) -> Vec<Drawable> {
        Vec::new()
    }

    fn to_json(&self) -> String{
        serde_json::to_string(self).expect("Pitfall should be JSON-serializable!")

    }

    fn query_json(&self, _query: &str, _args: &serde_json::Value) -> Result<String,QueryError>{
        Err(QueryError::NoSuchQuery)
    }
}


impl StateCore{
    fn from_config(config: &Pitfall) -> StateCore {
        StateCore{
            lives: 5,
            player: Player {x: 50 , y:50}
        }
    }
}