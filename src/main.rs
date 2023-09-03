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
        new_deck.append(&mut d2.cards);
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
        /* comparison is being made per individually for each position
        fix!
        let _comps = |i: usize| {
            self.cards[i] == Card::King && 
            (self.cards[(i + 1) % 5]) == Card::King &&
            (self.cards[(i + 2) % 5]) == Card::King
        }
        let test = self.cards
            .iter()
            .enumerate()
            .map(|(i, _)| comps(i)).collect::<Vec<bool>>();
            .fold(true, |acc, x| acc == x);
        */


        
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
    fn encode(&self, input: bool) -> Deck {
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
    // TODO figure out input
    fn encode(&self, input: bool) -> Deck {
        match self.date_alice {
            true => Deck { cards: vec![Card::K, Card::Q] },
            false => Deck { cards: vec![Card::Q, Card::K] },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Deck, Card::K, Card::Q};

    fn gen_starting_deck() -> Vec<Deck> {
        let mut base_deck = Deck::new(vec![K, K, K, Q, Q]);

        let mut permutations = Vec::with_capacity(5);

        for _ in 1..=5 {
            permutations.push(Deck::cyclic_shift(&mut base_deck, 1));
        }

        permutations
    }

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
}