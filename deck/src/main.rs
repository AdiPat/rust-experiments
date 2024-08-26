#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn build_complete_deck() -> Vec<String> {
    let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
    let values = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ];

    let mut deck = Vec::new();

    for suit in suits.iter() {
        for value in values.iter() {
            deck.push(format!("{} of {}", value, suit));
        }
    }

    deck
}

fn main() {
    let deck = Deck {
        cards: vec!["Ace of Spades".to_string(), "2 of Spades".to_string()],
    };
    println!("STANDARD DECK: {:?}", deck);

    let complete_deck: Deck = Deck {
        cards: build_complete_deck(),
    };

    println!("COMPLETE DECK: {:?}", complete_deck);
}
