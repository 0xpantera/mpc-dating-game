#[allow(warnings)]

fn main() {
    println!("Hello, world!");
}

#[derive( Clone, Debug, PartialEq)]
enum Card {
    Q,
    K,
}


#[derive(Clone, Debug, PartialEq)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new(v: Vec<Card>) -> Deck {
        Deck { cards: v }
    }

    /*
    This function will output a new deck that is comprised of three elements: 
    two input decks and a separator king card. 
    */
    fn join(d1: &mut Deck, d2: &mut Deck) -> Deck {
        let mut new_deck = d1.cards.clone();
        new_deck.push(Card::K);
        let mut snd_deck = d2.cards.clone();
        new_deck.append(&mut snd_deck);
        Deck { cards: new_deck }
    }

    /*
    Implement a method named decode as part of the deck struct.
    The method takes self (a deck of size five) as a parameter and outputs a Boolean value as follows: 
    if there are three kings in a row in our set of five cards we return true. 
    Note: a king in positions 1 and 5 are considered adjacent, 
    so if you have a king in positions 1, 2, and 5 we should return true.
     */
    fn decode(&self) -> bool {
        for (i, _) in self.cards.iter().enumerate() {
            if self.cards[i] == Card::K && 
                (self.cards[(i + 1) % 5]) == Card::K &&
                (self.cards[(i + 2) % 5]) == Card::K {
                    return true;
                }
        }
        return false;
    }

    /*
    Rotates the slice in-place such that the first mid elements of the slice
    move to the end while the last self.len() - mid elements move to the front. 
    After calling rotate_left, the element previously at index mid will become the first element in the slice.

    mid = 3
    [1, 2, 3, 4, 5]
    [1, 2, 3]
    len = 5
    len_minus_mid = 5 - 3 = 2
    [4, 5, 1, 2, 3]
     */

    fn cyclic_shift(d: &mut Deck, rotations: usize) -> Deck {
        d.cards.rotate_left(rotations);
        Deck::new(d.cards.clone())
    }
}


struct Alice {
    date_bob: bool,
}

impl Alice {
    fn new(date: bool) -> Alice {
        Alice { date_bob: date }
    }

    fn encode(self) -> Deck {
        match self.date_bob {
            true => Deck { cards: vec![Card::Q, Card::K] },
            false => Deck { cards: vec![Card::K, Card::Q] },
        }
    }
}

struct Bob {
    date_alice: bool,
}

impl Bob {
    fn new(date: bool) -> Bob {
        Bob { date_alice: date }
    }

    fn encode(self) -> Deck {
        match self.date_alice {
            true => Deck { cards: vec![Card::K, Card::Q] },
            false => Deck { cards: vec![Card::Q, Card::K] },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::{Alice, Bob, Deck, Card::K, Card::Q};

    #[test]
    fn shifts_deck() {
        let test_deck = vec![Deck::new(vec![K, K, Q, Q, K]), 
                             Deck::new(vec![K, Q, Q, K, K]), 
                             Deck::new(vec![Q, Q, K, K, K]), 
                             Deck::new(vec![Q, K, K, K, Q]), 
                             Deck::new(vec![K, K, K, Q, Q])];

        let permutations = gen_starting_deck();

        assert_eq!(permutations, test_deck);
    }
    
    #[test]
    fn decodes_deck() {
        let permutations = gen_starting_deck();
     
        let mut results = Vec::new();

        for deck in permutations.iter() {
            let decoded = deck.decode();
            results.push(decoded);
        }
        assert_eq!(results, vec![true, true, true, true, true]);
    }

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
    fn all_possibilities() {
        let mut alices = vec![Alice::new(true).encode(), Alice::new(false).encode()];
        let mut bobs = vec![Bob::new(true).encode(), Bob::new(false).encode()];

        let shifts = 1..5;
        let mut results: Vec<bool> = Vec::new();

        for shift in shifts {
            for a in alices.iter_mut() {
                for b in bobs.iter_mut() {
                    println!("Alice: {:?}", a);
                    println!("Bob: {:?}", b);
                    let mut joined_deck = Deck::join(a, b);
                    println!("Joined Deck: {:?}", joined_deck);
                    println!("Num Shifts: {}", shift);
                    Deck::cyclic_shift(&mut joined_deck, shift);
                    Deck::cyclic_shift(&mut joined_deck, shift);
                    let result = joined_deck.decode();
                    println!("Result: {}", result);
                    results.push(result);
                }
            }
        }


    }


    fn gen_starting_deck() -> Vec<Deck> {
        let mut base_deck = Deck::new(vec![K, K, K, Q, Q]);

        let mut permutations = Vec::with_capacity(5);

        for _ in 1..=5 {
            permutations.push(Deck::cyclic_shift(&mut base_deck, 1));
        }

        permutations
    }
}