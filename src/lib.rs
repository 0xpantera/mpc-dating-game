

#[derive( Clone, Debug, PartialEq)]
pub enum Card {
    Q,
    K,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Deck {
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
    pub fn join(d1: &mut Deck, d2: &mut Deck) -> Deck {
        let mut new_deck = d1.cards.clone();
        new_deck.push(Card::K);
        let mut snd_deck = d2.cards.clone();
        new_deck.append(&mut snd_deck);
        Deck { cards: new_deck }
    }

    /*
    The method takes self (a deck of size five) as a parameter and outputs a Boolean value as follows: 
    if there are three kings in a row in our set of five cards we return true. 
    Note: a king in positions 1 and 5 are considered adjacent.
    */
    pub fn decode(&self) -> bool {
        for (i, _) in self.cards.iter().enumerate() {
            if self.cards[i] == Card::K 
                && (self.cards[(i + 1) % 5]) == Card::K 
                && (self.cards[(i + 2) % 5]) == Card::K {
                    return true;
                }
        }
        return false;
    }

    /*
    Rotates the slice in-place such that the first mid elements of the slice
    move to the end while the last self.len() - mid elements move to the front. 
    After calling rotate_left, the element previously at index mid will become the first element in the slice.
    */
    pub fn cyclic_shift(d: &mut Deck, rotations: usize) -> Deck {
        d.cards.rotate_left(rotations);
        Deck::new(d.cards.clone())
    }
}


pub struct Alice {
    date_bob: bool,
}

impl Alice {
    pub fn new(date: bool) -> Alice {
        Alice { date_bob: date }
    }

    pub fn encode(self) -> Deck {
        match self.date_bob {
            true => Deck { cards: vec![Card::Q, Card::K] },
            false => Deck { cards: vec![Card::K, Card::Q] },
        }
    }
}

pub struct Bob {
    date_alice: bool,
}

impl Bob {
    pub fn new(date: bool) -> Bob {
        Bob { date_alice: date }
    }

    pub fn encode(self) -> Deck {
        match self.date_alice {
            true => Deck { cards: vec![Card::K, Card::Q] },
            false => Deck { cards: vec![Card::Q, Card::K] },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Deck, Card::K, Card::Q};

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

    fn gen_starting_deck() -> Vec<Deck> {
        let mut base_deck = Deck::new(vec![K, K, K, Q, Q]);

        let mut permutations = Vec::with_capacity(5);

        for _ in 1..=5 {
            permutations.push(Deck::cyclic_shift(&mut base_deck, 1));
        }

        permutations
    }
}