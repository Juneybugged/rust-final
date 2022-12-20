use std::env;
use rand::Rng;

/// Parses the command line arguments and returns a vector of strings representing the arguments.
fn parse_command_line_arguments() -> Vec<String> {
    // Parse command line arguments
    env::args().collect()
}

/// Splits the dice string into a tuple containing a vector of dice modifiers and a vector of dice rolls.
///
/// # Arguments
///
/// * `dice_string` - A string representing the dice to be rolled.
fn split_dice_string(dice_string: &str) -> (Vec<i32>, Vec<&str>) {
    // Split dice string into individual dice and dice modifiers
    let dice: Vec<&str> = dice_string.split("+").collect();
    let mut dice_modifiers = Vec::new();
    let mut dice_rolls = Vec::new();
    for die in dice {
        if let Ok(modifier) = die.parse::<i32>() {
            dice_modifiers.push(modifier);
        } else {
            dice_rolls.push(die);
        }
    }

    (dice_modifiers, dice_rolls)
}

/// Rolls a single die with the specified number of sides `size`, and returns the result.
///
/// # Arguments
///
/// * `count` - The number of dice to roll.
/// * `size` - The number of sides on the dice.
fn roll_dice(count: i32, size: i32) -> i32 {
    let mut roll = 0;
    if count > 1 {
        // Multiple dice of the same size, group results in parentheses
        print!("(");
        for i in 0..count {
            let result = rand::thread_rng().gen_range(1, size+1);
            roll += result;
            print!("{}", result);
            if i < count - 1 {
                print!(", ");
            }
        }
        println!(")");
    } else {
        // Single die, no grouping needed
        let result = rand::thread_rng().gen_range(1, size+1);
        roll += result;
        println!("Rolled a {} on a d{}", result, size);
    }
    roll
}

/// Splits a die string into a tuple containing the count and size of the dice.
///
/// # Arguments
///
/// * `die` - A string representing a single die, in the format "XdY", where X is the count and Y is the size.
fn split_die(die: &str) -> (i32, i32) {
    // Split die into count and size
    let parts: Vec<&str> = die.split("d").collect();
    let count: i32 = parts[0].parse().unwrap();
    let size: i32 = parts[1].parse().unwrap();
    (count, size)
}

fn main() {
    let dice_string = &parse_command_line_arguments()[1];
    let (dice_modifiers, dice_rolls) = split_dice_string(dice_string);

    // Initialize total roll to 0
    let mut total_roll = 0;

    // Iterate over dice
    for die in dice_rolls {
        let (count, size) = split_die(die);
        let roll = roll_dice(count, size);
        // Add roll to total
        total_roll += roll;
    }

    // Add dice modifiers to total roll
    for modifier in dice_modifiers {
        total_roll += modifier;
    }

    println!("Total roll: {}", total_roll);
}

