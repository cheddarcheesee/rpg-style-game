/*
This code WILL be published to The Hub (github). I'll be cleaning up any comments that are not related to the code.

Please use comments to explain the code :D

also i (cheese) apologize for any and all swearing that occurs in this code. it happens.
*/

use rand::Rng; // needed for randomization
use std::io; //	needed for user input
#[allow(dead_code)]	//	allows the use of code that doesnt have a use



// MAIN MENU
fn menu() { 
  println!("Welcome to the most totally 'heterosexual' game ever!");

    println!("--------");
    println!("MAIN MENU");
      println!("--------");
        println!("1. New Game");
        println!("2. Load Game"); 
        println!("3. Credits");
        println!("4. Quit");
      println!("-------");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line."); // inputs ^^
    if inp.trim() == "1" {
      new_game();
    } else if inp.trim() == "2" {
        load_game();
    } else if inp.trim() == "3"	{
        println!("
        LEAD DEVELOPER: Cheese
        ");
    } else if inp.trim() == "4" {
        println!("byebye ")
    } 
}  
// "load game" thing isn't my greatest priority now, that's a feature that'll probably be added later.
fn load_game() { 
	println!("not implemented yet!");
}

fn main() {
    menu();

}  

// CLASSES
struct Player { 
  name: String,
  health: i32,
  attack: i32,
  inventory: Vec<Tool>,
  spellbook: Vec<Spell>,
  exp: i32,
  lvl: i32,
  coins: i32,
} 

struct Spell {
  name: String,
  info: String,
  damage: i32,
  required_lvl: i32,
  }
impl Clone for Spell {
    fn clone(&self) -> Spell {
        Spell {
            name: self.name.clone(),
            info: self.info.clone(),
            damage: self.damage.clone(),
            required_lvl: self.required_lvl.clone(),
        }
    }
}

struct Tool {
  name: String,
  info: String,
  damage: i32,
  durability: i32,
  
}
impl Clone for Tool {
    fn clone(&self) -> Tool {
        Tool {
            name: self.name.clone(),
            info: self.info.clone(),
            damage: self.damage.clone(),
            durability: self.durability.clone(),
        }
    }
}

struct Food {
  name: String,
  info: String,
  heal: i32,
  servings: i32,
}

impl Clone for Food {
    fn clone(&self) -> Food {
        Food {
            name: self.name.clone(),
            info: self.info.clone(),
            heal: self.heal.clone(),
            servings: self.servings.clone(),
        }
    }
}

// LOCATIONS

fn mountain(player: &Player) {
  println!("mtn dew");
}

fn beach(player: &Player) {
println!("beatch!");
}

fn forest(player: &Player) {
  println!("You arrive at the forest and take in the scenery.");
  println!("{}", player.name);
  forest_events(player);
  
}

// EVENTS
fn wandering_trader(player: &Player) {
  println!("After walking for about 10 minutes, you encounter a person. They appear to be a wandering trader. However, all of their offers are way too overpriced!
  What do you do?
  1. Attack them
  2. Try to bargain
  ");
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Failed to read line.");
  if inp.trim() == "1" {
    println!("FIGHT!")
  }
  else if inp.trim() == "2" {
    println!("Commence bartering!")
  }
}
fn tree_mining(player: &Player) {
  println!("you walk up to a tree and begin punching it like this is minecraft.")
}
fn forest_events(player: &Player) {
  let mut rng = rand::thread_rng();
  let event_list = vec!["trader", "tree mining", "obama pyramid jumpscare"];
  // randomize from list 
  let event = event_list[rng.gen_range(0..event_list.len())];
  println!("{}", event);
  if event == "trader" {
    wandering_trader(player)
  }
  else if event == "tree mining" {
    tree_mining(player)
  }
  else if event == "obama pyramid jumpscare" {
    println!("the obama pyramid runs towards you! oh no! (i got a bit lazy, i apologize. this will be cleaned up later.)")
  }
}
// NEW GAME
fn new_game() { 
  // VARIABLE DECLARATIONS
  let fireball = Spell {
    name: String::from("Fireball"),
    info: String::from("A small burst of fire."),
    damage: 10,
    required_lvl: 1,
  };
  let stone_sword = Tool {
    name: String::from("Stone Sword"),
    info: String::from("A weak yet reliable sword forged by a blacksmith."),
    damage: 15,
    durability: 50,
  };
  let apple = Food {
    name: String::from("Apple"),
    info: String::from("A small and healthy snack."),
    heal: 10,
    servings: 1,
  };
  let mut randomguy = Player { // assigns "randomguy" to the player class
    attack: 10,
    name: String::new(),
    health: 100,
    inventory: Vec::new(),
    spellbook: Vec::new(),
    exp: 0,
    lvl: 1,
    coins: 2,
  
  }; 
  let stone_axe = Tool {
    name: String::from("Stone Axe"),
    info: String::from("e"),
    damage: 20, 
    durability: 50
  };
  // NAMING
  println!("Enter your character's name");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line.");
  randomguy.name = input.trim().to_string();
  // add spells + items to inventory/spellbook
  randomguy.inventory.push(stone_sword.clone());
  randomguy.inventory.push(stone_axe.clone());

  randomguy.spellbook.push(fireball);
  
  println!("----------------");
  println!(" "); // opening scene
  println!("You slowly open your eyes. You seem to have woken up on a warm beach. You decide to inspect your tools, a {} and a {}. You have no idea how you got here or where you are", stone_sword.name, stone_axe.name);
  println!("In your peripheral vision, you see the beginning of a forest to your right, a small, beach-dwelling town on your left, and a stretch of tall mountains in front of you.

  ");
  println!("Where do you go?
1. Forest
2. Beach Town
3. Mountains
  ");
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Failed to read line.");
  if inp.trim() == "1" {
    forest(&randomguy)
  }
  else if inp.trim() == "2" {
    beach(&randomguy)
  }
  else if inp.trim() == "3" {
    mountain(&randomguy)
  }
  else {
    println!("invalid input D:")
  }
  
} 
// current end of code.
