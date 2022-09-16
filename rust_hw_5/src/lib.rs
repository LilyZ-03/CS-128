/// TODO: Implement the mark_cards function
/// Marks every 3rd card by adding M to the front of the card's 
/// string representation (irrespective of if it already has a M or not)
/// Make sure to derefence cards before changing it
pub fn mark_cards(cards: &mut Vec<String>) {
    for i in 0..cards.len() {
        if i % 3 == 2 {
            cards[i] = String::from("M") + &cards[i]; 
        }
    }
}

/// TODO: Implement the perfect_bridge function
/// Takes a Vector of strings representing a deck of cards (will always have an even number) 
/// Divides the deck into two parts at the middle, then shuffles the cards 
/// Such that the first card of the first half resides in the 0th position 
/// and the first card of the second hald resides in the 1th position
/// And the second card of the first half resides in the 3rd position etc.
/// After shuffling return the deck
pub fn perfect_bridge(cards: &Vec<String>) -> Vec<String> {
    let mut shuffled: Vec<String> = Vec::new();
    let half_len: usize = cards.len()/2;
    for i in 0..half_len {
        shuffled.push(cards[i].clone());
        shuffled.push(cards[half_len + i].clone());
    }
    shuffled
}

/// TODO: Implement the runner function
/// Takes a vector of strings representing a deck of cards
/// Shuffle the cards, then mark the cards, and finally shuffle them again
pub fn runner(cards: &mut Vec<String>) {
    *cards = perfect_bridge(cards);
    mark_cards(cards);
    *cards = perfect_bridge(cards);
}