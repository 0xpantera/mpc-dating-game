use mpc_dating_game::{Alice, Bob, Deck};
use std::io;

fn main() {
    println!("
    ####################################################
    ####  Hello Alice, would you date Bob? (Y/n)  ######
    ####################################################
    ");
    let mut input = String::new();
    
    let _ = match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Alice has answered."),
        Err(e) => println!("Weird anser from Alice: {}", e)
    };

    let mut alice = match input.to_lowercase().trim() {
        "y"  => Alice::new(true).encode(),
        _ => Alice::new(false).encode(),
    };
    println!("Alice's deck: {:?}", &alice);

    println!("
    ####################################################
    ####  Hello Bob, would you date Alice? (Y/n)  ######
    ####################################################
    ");
    let mut input = String::new();
    
    let _ = match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Bob has answered."),
        Err(e) => println!("Weird anser from Bob: {}", e)
    };

    let mut bob = match input.to_lowercase().trim() {
        "y"  => Bob::new(true).encode(),
        _ => Bob::new(false).encode(),
    };
    println!("Bob's deck: {:?}", &bob);

    let mut joined_deck = Deck::join(&mut alice, &mut bob);

    println!("
    ################################################################
    ### Alice, specify the number of times to shuffle the deck: ####
    ################################################################
    ");

    let mut shift_input = String::new();
    let _ = io::stdin().read_line(&mut shift_input).unwrap();
    let alice_shifts = shift_input.trim().parse::<usize>().unwrap();

    Deck::cyclic_shift(&mut joined_deck, alice_shifts);

    println!("
    ################################################################
    ### Bob, specify the number of times to shuffle the deck: ######
    ################################################################
    ");

    let mut shift_input = String::new();
    let _ = io::stdin().read_line(&mut shift_input).unwrap();
    let bob_shifts = shift_input.trim().parse::<usize>().unwrap();

    Deck::cyclic_shift(&mut joined_deck, bob_shifts);

    match joined_deck.decode() {
        true => println!("It's a match!"),
        false => println!("No match!"),
    }
}

