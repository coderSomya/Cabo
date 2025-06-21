pub struct Player{
    // unique id of the player,
    player_id: String,
    // name of the player
    name: String,
    // password of the player
    password: String,
    // the game that he is currently in
    game_id: Option<String>,
    // postion of the player in the table in the current game
    player_positio: Option<u32>,
    // the set of cards the player has
    cards: Option<Card[]>,
}

impl Player{
    // a player wants to `play` the card with id as `id`
    pub fn play(&self, id: u32) -> Result<(), String>{
        todo!()
    }

    // a player wants to `burn` the card with id `which_id` of a player with id `whose_id`
    pub fn burn(&self, whose_id: u32, which_id: u32) -> Result<(), String>{
        todo!()
    }

    // a player wants to `swap` the card with id `which_id` with a player with if `whom_id`
    pub fn swap_own(&self, whom_id: u32, which_id: u32) -> Result<(), String>{
        todo!()
    }

    // a player want to `swap` the cards `card1` of `player1` and `card2` of `player2`
    pub fn swap_others(&self, player1_id: u32, player2_id: u32, card1_id: u32, card2_id: u32){
        todo!()
    }

    // a player wants to `see` the card with id `which_id` of a player with id `whose_id`
    pub fn see(&self, whose_id: u32,  which_id: u32) -> Result<(), String>{
        todo!()
    }

    // a player wants to call "cabo"
    pub fn call(&self){
        todo!()
    }
}