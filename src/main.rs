use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];
    let goto_mode = mode.as_str();

    println!("Summoning the Caster in {} mode.", mode);

    match goto_mode{
        "--norm"|"-n"=>roll(),
        "--help"|"-h"|"-?"=>help(),
        "--gods"|"-g"=>gods(),
        &_ => todo!(),
    }
}

fn roll() {
    println!{"Welcome to the Magic Roller!"};
    
    let x = rand::thread_rng().gen_range(0..=20);
    let y = rand::thread_rng().gen_range(1..=4);
    let quality: String;
    let modifier: String;

    match x {
        1..=4 => quality = "A-Rank (50)".to_string(),
        5..=8 => quality = "B-Rank (40)".to_string(),
        9..=12 => quality = "C-Rank (30)".to_string(),
        13..=14 => quality = "D-Rank (20)".to_string(),
        _ => quality = "E-Rank (10)".to_string(),
    }
    
    match y {
        1 => modifier = "+ (x2)".to_string(),
        2 => modifier = "++ (x3)".to_string(),
        3 => modifier = "+++ (x4)".to_string(),
        _ => modifier = "(No modifier)".to_string(),
    }
    println!("You have {} {} circuits", quality, modifier);
}

fn help() {
    println!("Type-Moon Style Magic Circuit Randomizer.\n Switches:\n --norm/-n: Runs a chart based on the Parameter Rules from the Type-Moon Fandom.\n --gods/-g: Runs a much higher chart equivalent to the power level possibly found during the Age of Gods.\n --help/-?/-h: Displays this message.");
}

fn gods() {
    println!("Regression to the Age of Gods Roller.\n This roller should give a higher output.");
    
    let x = rand::thread_rng().gen_range(1..=20);
    let quality: String;
    let quantity: i32;
    
    match x{
        1 => quality = "EX-Class".to_string(),
        2..=5 => quality = "A-Class".to_string(),
        6..=10 => quality = "B-Class".to_string(),
        11..=15 => quality = "C-Class".to_string(),
        _ => quality = "D-Class".to_string(),
    }
    
    match quality.as_str() {
        "EX-Class" => quantity = rand::thread_rng().gen_range(1..=100) + rand::thread_rng().gen_range(1..=100) + rand::thread_rng().gen_range(1..=100) + 103,
        "A-Class" => quantity = rand::thread_rng().gen_range(1..=100) + rand::thread_rng().gen_range(1..=100) + 102,
        "B-Class" => quantity = rand::thread_rng().gen_range(1..=100) + 101,
        "C-Class" => quantity = rand::thread_rng().gen_range(1..=75) + 31,
        "D-Class" => quantity = rand::thread_rng().gen_range(1..=50) + 21,
        &_ => todo!()
    }
    
    println!("{} circuits of {} rank.", quantity, quality);
}