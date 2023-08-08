use std::sync::Arc;
use std::{io, vec};
use std::collections::HashMap;
use ::Nod::Network;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time;
use Nod::Agency;    

/////////////////////
/// Card Section ///
///////////////////
// Declare Card enum
#[derive(PartialEq, Eq, Hash)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    OneEye
}

impl Card {
    fn new_number (&self, number: i64) -> i64 {
        match self {
            Card::Ace => number + 1,
            Card::Two => number + 2,
            Card::Three => number + 3,
            Card::Four => number + 4,
            Card::Five => number + 5,
            Card::Six => number + 6,
            Card::Seven => number + 7,
            Card::Eight => number,
            Card::Nine => number,
            Card::Ten => number - 10,
            Card::Jack => number + 10,
            Card::Queen => number + 10,
            Card::King => number + 10,
            Card::OneEye => 99
        }
    }

    fn num_form (&self) -> i32 {
        match self {
            Card::Ace => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::OneEye => 0
        }
    }
}

// Implement a string format for each card
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Ace => write!(f, "Ace"),
            Card::Two => write!(f, "Two"),
            Card::Three => write!(f, "Three"),
            Card::Four => write!(f, "Four"),
            Card::Five => write!(f, "Five"),
            Card::Six => write!(f, "Six"),
            Card::Seven => write!(f, "Seven"),
            Card::Eight => write!(f, "Eight"),
            Card::Nine => write!(f, "Nine"),
            Card::Ten => write!(f, "Ten"),
            Card::Jack => write!(f, "Jack"),
            Card::Queen => write!(f, "Queen"),
            Card::King => write!(f, "King"),
            Card::OneEye => write!(f, "OneEye")
        }
    }
}

// Implement clone for Card
impl Clone for Card {
    fn clone(&self) -> Card{
        match self {
            Card::Ace => Card::Ace,
            Card::Two => Card::Two,
            Card::Three => Card::Three,
            Card::Four => Card::Four,
            Card::Five => Card::Five,
            Card::Six => Card::Six,
            Card::Seven => Card::Seven,
            Card::Eight => Card::Eight,
            Card::Nine => Card::Nine,
            Card::Ten => Card::Ten,
            Card::Jack => Card::Jack,
            Card::Queen => Card::Queen,
            Card::King => Card::King,
            Card::OneEye => Card::OneEye
        }
    }
}



//////////////////////
// PLAYERS SECTION //
////////////////////
/// 

trait Turn {
    fn take_turn(&mut self, current_number: i64, discard: HashMap<Card, usize>, direction: i64, num_players: usize) -> Card;
}

// Define the real person player
#[derive(Default)]
struct Person {
    hand: Vec<Card>,
    losses: u32
}

///// Person methods /////

impl Person {
    fn play_card(&mut self) -> Card{
        let mut choose: bool = true;
        let mut selection: usize = 1;
        while choose {
            let mut read = String::new();
            io::stdin().read_line(&mut read).expect("Invalid input");
            let input = read.trim();
            let see = vec![1,2,3];
            selection = match input{
                "1" => 1,
                "2" => 2,
                "3" => 3,
                _ => 4
            };
            if see.contains(&selection) {
                choose = false;
            }
        }
        let choice = self.hand.remove(selection - 1);
        return choice.clone();
    }

    // Take turn
    fn take_turn(&mut self, current_number: i64) -> Card {
        println!("The current number is: {}\nYou have cards ({},{},{})\nType 1,2,3 to pick a card)", current_number, self.hand[0], self.hand[1], self.hand[2]);
        let choice: Card = self.play_card();
        return choice;
    }

}

//// Basic Bot ////
#[derive(Default)]
struct BasicBot {
    hand: Vec<Card>,
    losses: u32,
    priorities: [usize; 14]
}

impl BasicBot {
    fn randomize(&mut self) -> i32 {
        for i in 0..14 {
            self.priorities[i] = i;
        }
        let mut rng = thread_rng();
        self.priorities.shuffle(&mut rng);
        return 0;
    }

    fn play_card(&mut self, current_number: i64) -> Card {
        for card in self.priorities {
            for hcard in 0..3 {
                if self.hand[hcard].num_form() == card as i32 && self.hand[hcard].new_number(current_number) <= 99{
                    return self.hand.remove(hcard);
                }
            }
        }
        return self.hand.remove(0);
    }

    fn take_turn(&mut self, current_number: i64) -> Card {
        let choice = self.play_card(current_number);
        return choice;
    }
}


//// Nod Bot ////
struct BotNod {
    hand: Vec<Card>,
    losses: u32,
    brain: Network
}

impl BotNod {
    // Select card
    fn play_card(&mut self, current_number: i64, discard: HashMap<Card, usize>, direction: i64, num_players: usize) -> Card {
        let mut inputs: Vec<f64> = vec![direction as f64, 1.0 / (num_players as f64)];
        let mut used: Vec<f64> = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Jack, Card::Queen, Card::King, Card::OneEye].iter().map(|x| (*discard.get(x).unwrap() as f64) / 4.0).collect();
        let mut hand: Vec<f64> = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Jack, Card::Queen, Card::King, Card::OneEye].iter().map(|x| (self.hand.iter().filter(|&y| y == x).count() as f64 + 1.0).ln()).collect();
        inputs.append(&mut used);
        inputs.append(&mut hand);
        inputs.push(current_number as f64 / 99.0);
        if current_number > 89 {
            if current_number > 98 {
                inputs.push(1.0)
            } else {
                inputs.push(0.0)
            }
            inputs.push(1.0);
            inputs.push((current_number as f64 - 90.0) / 9.0);
        } else {
            inputs.push(0.0);
            inputs.push(0.0);
            inputs.push(0.0)
        }
        inputs.push(match current_number {
            x if x < 80 => -1.0,
            x if x > 79 && x < 90 => 0.0,
            x if x > 89 => 1.0,
            _ => 0.0
        });
        
        let mut outputs: Vec<f64> = self.brain.process(inputs);
        let mut big: usize = 0;
        for i in 0..outputs.len() {
            for j in 0..outputs.len() {
                if outputs[j] > outputs[big] {
                    big = j;
                }
            }
            for j in 0..3 {
                if self.hand[j].num_form() == big as i32 {
                    return self.hand.remove(j);
                }
            }
            outputs[big] = 0.0;
        }
        return self.hand.remove(0);
    }

    // Take turn
    fn take_turn(&mut self, current_number: i64, discard: HashMap<Card, usize>, direction: i64, num_players: usize) -> Card {
        self.play_card(current_number, discard, direction, num_players)
    }
}


///////////////////
/// Player Set ///
/////////////////

enum Player {
    Human(Person),
    Bot(BasicBot),
    Nod(BotNod)
}

impl Turn for Player {
    fn take_turn(&mut self, current_number: i64, discard: HashMap<Card, usize>, direction: i64, num_players: usize) -> Card {
        let card: Card = match self {
            Player::Human(f) => f.take_turn(current_number),
            Player::Bot(f) => f.take_turn(current_number),
            Player::Nod(f ) => f.take_turn(current_number, discard, direction, num_players)
        };
        return card;
    }
}

impl Player {
    fn take_card(&mut self, card: Card) {
        match self {
            Player::Human(f) => f.hand.push(card),
            Player::Bot(f) =>f.hand.push(card),
            Player::Nod(f) => f.hand.push(card)
        }
    }

    fn rand(&mut self) {
        match self {
            Player::Human(_) => 0,
            Player::Bot(f) => f.randomize(),
            Player::Nod(_) => 0
        };

    }

    fn lose(&mut self) {
        match self {
            Player::Human(f) => f.losses += 1,
            Player::Bot(f) => f.losses += 1,
            Player::Nod(f) => f.losses += 1
        };
    }

    fn clear_hand(&mut self) {
        match self {
            Player::Human(f) => f.hand.clear(),
            Player::Bot(f) => f.hand.clear(),
            Player::Nod(f) => f.hand.clear()
        }
    }

    fn loss_count(&self) -> u32 {
        match self {
            Player::Human(f) => f.losses,
            Player::Bot(f) => f.losses,
            Player::Nod(f) => f.losses
        }
    }
}

////////////
/// Deck //
//////////
fn build_deck() -> Vec<Card>{

    let mut deck: Vec<Card> = Vec::new();
    for _i in 0..2 {
        for c in [Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Queen, Card::King, Card::OneEye] {
        deck.push(c)
        };
    };
    for _i in 0..2 {
        for c in [Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Jack, Card::Queen, Card::King] {
            deck.push(c)
        };
    };
    return deck;
}


/////////////
/// GAME ///
///////////

// Main game run sequence, will return the losing player
fn run_game(players: &mut Vec<&mut Player>, first_player: i64) -> i64{
    let num_players: usize = players.len();
    if players.len() == 0{
        return -1;
    }
    let mut deck: Vec<Card> = build_deck();
    let mut discard: HashMap<Card, usize> = HashMap::new();
    for c in [Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
    Card::Jack, Card::Queen, Card::King, Card::OneEye] {
        discard.insert(c, 0);
    }
    let mut current_player = first_player;
    let mut current_number: i64 = 0;
    let mut direction: i64 = 1;
    let mut rng = rand::thread_rng();
    let mut end: bool = false;

    deck.shuffle(&mut rng);

    for i in 0..num_players {
        players[i].clear_hand();
    }

    for _i in 0..3 {
        for j in 0..num_players {
            players[j].take_card(deck.remove(0))
        }
    }

    while !end {
        let played = players[current_player as usize].take_turn(current_number, discard.clone(), direction, num_players);
        let play_copy = played.clone();
        discard.insert(played, discard.get(&play_copy).unwrap_or(&0) + 1);
        if let Card::Eight = play_copy {
        direction = direction * -1;
        }
        current_number = play_copy.new_number(current_number);
        if current_number > 99 {
            end = true;
        } else {
        current_player = current_player + direction;
        if current_player >= num_players as i64{
            current_player = 0;
        } else if current_player <= -1 {
            current_player = num_players as i64 - 1;
        }
        players[current_player as usize].take_card(deck.remove(0));
        if deck.len() == 0 {
            for key in discard.keys() {
                for _ in 0..*discard.get(key).unwrap() {
                    deck.push(key.clone())
                }
            }
            deck.shuffle(&mut rng)
        }
        }
    }
    players[current_player as usize].lose();
    return current_player;

}

// Create a function that will run two sets of players against each other, facing each player against each player on the other set N times
fn run_team_round_robin_1v1(mut team1: Vec<Player>, mut team2: Vec<Player>, num_games: usize) -> Vec<Vec<usize>> {
    for i in 0..team1.len() {
        for j in 0..team2.len() {
            for k in 0..num_games {
                let mut players = vec![&mut team1[i], &mut team2[j]];
                run_game(&mut players, k as i64 % 2);
            }
        }
    }
    let team1_losses: Vec<usize> = team1.iter().map(|f| f.loss_count() as usize).collect::<Vec<usize>>();
    let team2_losses: Vec<usize> = team2.iter().map(|f| f.loss_count() as usize).collect::<Vec<usize>>();
    
    vec![team1_losses, team2_losses]
}


fn main() {
    let start = time::Instant::now();
    //let root = Nod::build_typical_model(vec![30,2,14], String::from("relu"), String::from("sigmoid"));
    let decp_root = Nod::model_from_txt(String::from("decp_3_types"), String::from("decp_3_weights"), String::from("decp_3_connections"), String::from("tanh"), String::from("sigmoid"));
    //let mut autobots = Nod::build_agency_from_root(root.clone(), 25, true);
    let mut decepticons = Nod::build_agency_from_root(decp_root, 150, false);
    let mut results: Vec<Vec<usize>> = vec![];
    for i in 0..200 {
        decepticons.genetic_generation(75, 10, 35, 30, 5, 0.005, 0.2);
        //let autobot_team: Vec<Player> = autobots.networks_iter().map(|f| Player::Nod(BotNod { hand: vec![], losses: 0, brain: f })).collect();
        let decepticon_team: Vec<Player> = decepticons.networks_iter().map(|f| Player::Nod(BotNod { hand: vec![], losses: 0, brain: f })).collect();
        /*
        let mut basic = Player::Bot(BasicBot { hand: vec![], losses: 0, priorities: [0,0,0,0,0,0,0,0,0,0,0,0,0,0]});
        let mut basic2 = Player::Bot(BasicBot { hand: vec![], losses: 0, priorities: [0,0,0,0,0,0,0,0,0,0,0,0,0,0]});
        let mut basic3 = Player::Bot(BasicBot { hand: vec![], losses: 0, priorities: [11,12,13,7,6,5,4,3,2,1,0,8,9,10]});
        let mut basic4 = Player::Bot(BasicBot { hand: vec![], losses: 0, priorities: [11,12,13,7,6,5,4,3,2,1,0,8,9,10]});
        let mut basic5 = Player::Bot(BasicBot { hand: vec![], losses: 0, priorities: [11,12,13,7,6,5,4,3,2,1,0,8,9,10]});
        basic.rand();
        basic2.rand();
        basic3.rand();
        basic4.rand();
        basic5.rand();
        */
        let mut basic6 = Player::Bot(BasicBot { hand: vec![], losses: 0, priorities: [11,12,13,7,6,5,4,3,2,1,0,8,9,10]});

        results = run_team_round_robin_1v1(vec![basic6], decepticon_team, 150);
        //let auto_inversion = results[0].iter().map(|f| 1.0 / f.clone() as f64).collect();
        //autobots.reorder(auto_inversion);
        /*
        if i == 199 {
            let mut j = 0;
            for a in autobots.networks_iter() {
                if j == 0 {
                    a.model_to_txt(String::from("auto"));
                    j = 1;
                }
            }
        }
        autobots.genetic_generation(15, 3, 4, 3, 5, 0.01, 0.5);
        */
        let decp_inversion = results[1].iter().map(|f| 1.0 / f.clone() as f64).collect();
        decepticons.reorder(decp_inversion);
        if i == 199 {
            let mut j = 0;
            for a in decepticons.networks_iter() {
                if j == 0 {
                    a.model_to_txt(String::from("decp_7"));
                    j = 1;
                }
            }
        }
        if i % 10 == 0 {
            let mut top_loss: f64 = 0.0;
            for j in 0..5 {
                top_loss += decepticons.agents[j].score;
            }

            println!("Generation {}\nDecp top 5 avg loss rate: {}", i, top_loss / 5.0 / results[1].len() as f64 / 150.0);
            println!("Time Since start: {:?}",time::Instant::now().duration_since(start));
        }
    }
    
    let mut auto_loss = 0;
    let mut decp_loss = 0;
    for i in 0..results[0].len() {
        auto_loss += results[0][i];
    };
    for i in 0..results[1].len() {
        decp_loss += results[1][i];
    };
    let mut top_loss: f64 = 0.0;
    for j in 0..5 {
        top_loss += decepticons.agents[j].score;
    }
    println!("Decp top 5 avg loss rate: {}", top_loss / 5.0 / results[1].len() as f64 / 150.0);
    println!("The autobots lost a total of {} games, the decpticons lost a total of {}", auto_loss, decp_loss);
    println!("The total loss vectors\nAutobots:\n{:?}\nDecepticons:\n{:?}", results[0], results[1]);
    println!("Time Since start: {:?}",time::Instant::now().duration_since(start));
}