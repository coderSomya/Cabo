use crate::player::Player;


pub struct Move{
    // which player id's move is this
    player_id: String,
    // what card did he play
    card: Card,
    // the burned card in this move
    burned_card: Option<Card>,
}

pub enum Status{
    // game is yet to start
    NotStarted,
    // someone is looking at their card and thinking what to do
    CardPlaying,
    // card has been played and everyone's waiting for burn or power execution
    CardPlayed,
    // game is over
    Finished
}

impl PartialEq for Status{
    fn eq(&self, other: &Self) -> bool {
        other
    }
}
pub struct Game{
    // the unique id of the game
    game_id: String,
    // the list of the players in this game
    players: Vec<Player>,
    // the current move
    current_move: Option<Move>,
    // the status of the game
    status: Status,
    // the cards that have been played so far
    played_deck: Option<Vec<Card>>,
    // the cards from which we are playing
    playing_deck: Option<Vec<Card>>
}


impl Game{

    // `game_starter` initiates a game 
    pub fn new(&self, game_starter: Player) -> Game{
        Self{
            game_id: "123".to_string(), //todo! -> replace with unique id
            players: vec![game_starter],
            current_move: None,
            status: Status::NotStarted,
            played_deck: None,
            playing_deck: None,
        }
    }

    // start the game
    pub fn start(&mut self){
        self.status = Status::CardPlaying
        
        (playing_deck: Vec<Cards>, player_cards: Vec<Vec<Card>>) = Deck::get_decks(self.players.len());
        // distribute the cards
    }

    // add a `new_player` to the game
    pub fn add_player(&mut self, new_player: Player){
        if self.status == Status::NotStarted {
            self.players.push(new_player);
        }
    }

    // todo! -> how the fuck are we going to remove a player??

    fn get_winner(&self) -> Player{
        // check the minimum score
        todo!()
    }

    pub fn end(&self) -> Player{
        return self.get_winner();
    }

    // penalize a player for late burn
    fn penalize(&self, player_id: u32){

    }

    
}

