struct Creature {
    name: String,
    health: i32,
    max_health: i32,
    dmg: i32,
}

impl Creature {
    fn new(name: String, max_health: i32, dmg: i32) -> Creature {
        Self {
            name,
            health: max_health,
            max_health,
            dmg,
        }
    }

    fn print_status(&self) {
        if self.health <= 0 {
            println!("{} is dead!", self.name);
        } else {
            println!(
                "{} has {}/{} health and deals {} damage.",
                self.name, self.health, self.max_health, self.dmg
            )
        }
    }

    fn fight(&mut self, other: &mut Creature) {
        self.health -= other.dmg;
        other.health -= self.dmg;
    }
}

fn main() {
    println!();
    let mut minotaur = Creature::new("minotaur".to_string(), 8, 3);
    let mut goblin = Creature::new("goblin".to_string(), 2, 5);
    minotaur.print_status();
    goblin.print_status();
    println!();
    println!("{} is fighting {}...", minotaur.name, goblin.name);
    minotaur.fight(&mut goblin);
    println!();
    minotaur.print_status();
    goblin.print_status();
}
