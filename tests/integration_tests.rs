use mpc_dating_game::{self, Alice, Bob, Deck};
use rand::{self, Rng};


/*
Generate inputs for Alice and Bob.
1. Encode the inputs to decks.
2. Join the decks, with the Alice deck first.
3. Shuffle the new deck with a random shift.
4. Shuffle the deck again with another random shift.
5. Decode the resulting deck and check that you get the required output (match/no match). 
*/
#[test]
fn workflow() {
    let mut alice = Alice::new(true).encode();
    let mut bob = Bob::new(true).encode();

    let mut joined_deck = Deck::join(&mut alice, &mut bob);

    let fst_shift = rand::thread_rng().gen_range(1..=5);
    let snd_shift = rand::thread_rng().gen_range(1..=5);

    Deck::cyclic_shift(&mut joined_deck, fst_shift);
    Deck::cyclic_shift(&mut joined_deck, snd_shift);

    let result = joined_deck.decode();

    assert!(result);

}

#[test]
fn all_combinations() {
    let mut alices = vec![Alice::new(true).encode(), Alice::new(false).encode()];
    let mut bobs = vec![Bob::new(true).encode(), Bob::new(false).encode()];

    let shifts = 1..5;
    let mut results: Vec<bool> = Vec::new();

    for shift in shifts {
        for a in alices.iter_mut() {
            for b in bobs.iter_mut() {
                let mut joined_deck = Deck::join(a, b);
                Deck::cyclic_shift(&mut joined_deck, shift);
                Deck::cyclic_shift(&mut joined_deck, shift);
                let result = joined_deck.decode();
                results.push(result);
            }
        }
    }

    let expected = vec![true, false, false, false, true, false, false, false, 
                    true, false, false, false, true, false, false, false];

    assert_eq!(expected, results);
}
