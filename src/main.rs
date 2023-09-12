use mpc_dating_game::{Alice, Bob, Deck};
use std::io;

fn main() {
    println!("
    ####################################################
    ####  Hello Alice, would you date Bob? (Y/n)  ######
    ####################################################
    ");

    let mut alice = validate_input("alice");
    println!("Alice's deck: {:?}", &alice);

    println!("
    ####################################################
    ####  Hello Bob, would you date Alice? (Y/n)  ######
    ####################################################
    ");

    let mut bob = validate_input("bob");
    println!("Bob's deck: {:?}", &bob);

    let mut joined_deck = Deck::join(&mut alice, &mut bob);

    println!("
    ################################################################
    ### Alice, specify the number of times to shuffle the deck: ####
    ################################################################
    ");

    validate_shifts(&mut joined_deck);

    println!("
    ################################################################
    ### Bob, specify the number of times to shuffle the deck: ######
    ################################################################
    ");

    validate_shifts(&mut joined_deck);

    match joined_deck.decode() {
        true => println!("It's a match!"),
        false => println!("No match!"),
    }
}

// TODO: matching between bob and alice is fucked
fn validate_input(player: &str) -> Deck {
    let mut input = String::new();
    
    let _ = io::stdin().read_line(&mut input).unwrap();

    let deck = match player {
        "alice" =>
            match input.to_lowercase().trim() {
                "y"  => Alice::new(true).encode(),
                _ => Alice::new(false).encode(),
            }
        "bob" =>
            match input.to_lowercase().trim() {
                "y" => Bob::new(true).encode(),
                _ => Bob::new(false).encode(),
            }
        _ => Bob::new(true).encode()
    };
    deck
}


fn validate_shifts(d: &mut Deck) -> () {
    let mut shift_input = String::new();
    let _ = io::stdin().read_line(&mut shift_input).unwrap();
    let shifts = shift_input.trim().parse::<usize>().unwrap_or(0);
    
    let validated_shifts = match shifts {
        0..=5 => shifts,
        _ => 0,
    };


    Deck::cyclic_shift(d, validated_shifts);
}