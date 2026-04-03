
struct Player{
    id:u32,
    name:String,
    balance:u64,
}

impl Player{
    fn new(players:&mut Vec<Player>,id:u32,name:String)->Result<(), String>{
        for player in players.iter_mut(){
            if player.id==id{
                return Err("Player id already exists".to_string());
            }
        }
        players.push(Player { id:id, name:name, balance: 0 });
        return Ok(());
        
    }

    fn print_players(players:&Vec<Player>){
        for player in players.iter(){
            println!("Player id: {} - Player name: {} - Player balance: {}",player.id,player.name,player.balance)
        }
    }

    fn add_funds(players:&mut Vec<Player>,id:u32,amount:u64)->Result<(),String>{
        for player in players.iter_mut(){
            if player.id==id{
                player.balance+=amount;
                return Ok(());
            }
        }
        Err("Player id not found".to_string())
    }
}
#[derive(PartialEq)]
#[derive(Debug)]
enum LotteryState{
    Open,Drawing,Closed
}

struct Lottery{
    state:LotteryState,
    pool:u64,
    ticket_price:u64,
    players:Vec<u32>,
}


impl Lottery{
    fn new()->Lottery{
        println!("New Lottery Created");
        Lottery { state: LotteryState::Open, pool: 0, ticket_price: 48, players: Vec::new()}
    }

    fn lottery_state(lottery:&Lottery){
        println!("Lottery State: {:?}",lottery.state);
        println!("Lottery Pool amount: {}",lottery.pool);
        println!("Id's joined:    ");
        for id in lottery.players.iter(){
            print!("id: {}...",id)
        }
    }

    fn add_player_lottery(lottery:&mut Lottery,players:&mut Vec<Player>,id:u32)->Result<(),String>{
    
        if lottery.state == LotteryState::Drawing{
            return Err("Lottery already drawing, cannot add player".to_string());
        }
        if lottery.state == LotteryState::Closed{
            return Err("Lottery already closed, cannot add player".to_string());
        }

        for id_lottery in lottery.players.iter(){
            if *id_lottery == id{
                        return Err("Player already entered lottery".to_string());
                }
            }

        for player in players.iter_mut(){
            if player.id == id{
                if player.balance<lottery.ticket_price{
                    return Err("Not enough balance in player account".to_string());
                }
                else {
                    player.balance -= lottery.ticket_price;
                    lottery.pool+=lottery.ticket_price;
                    lottery.players.push(id);
                    return Ok(());
                    
                    
                }
                
            }
            
        }
      Err("Player id not found in vector".to_string())
}
    
    fn start_drawing(lottery:&mut Lottery)->Result<(),String>{
        if lottery.state == LotteryState::Drawing{
            return Err("Lottery already drawing".to_string());
        }
        if lottery.state == LotteryState::Closed{
            return Err("Lotter already closed".to_string());
        }
        if lottery.players.len()==0{
            return Err("Lottery has no players, please add players".to_string());
        }
        lottery.state=LotteryState::Drawing;
        return Ok(());
    }

    fn close_lottery(lottery:&mut Lottery)->Result<(),String>{
        if lottery.state == LotteryState::Open{
            return Err("Lottery is still Open, cannot close".to_string());
        }
        if lottery.state == LotteryState::Closed{
            return Err("Lottery already closed".to_string());
        }

        lottery.state = LotteryState::Closed;
        return Ok(());
    }

    fn select_winner(lottery:&mut Lottery,players:&mut Vec<Player>){
        let mut sum = 0;
        for player in players.iter(){
            sum += player.id as u64
        }
        let winner_index = (lottery.pool as u64 + sum)%lottery.players.len() as u64;
        println!("lottery Pool: {},lottery players lenght: {}",lottery.pool,lottery.players.len());
        let winner_id = lottery.players[winner_index as usize];
        println!("Lottery winning id is: 🤑🤑🤑 -----> {}",winner_id);
        for player in players.iter_mut(){
            if player.id == winner_id{
                player.balance+=lottery.pool;
                lottery.pool=0;
            }
        }
    }
    }
        
    



fn main(){

    let mut players: Vec<Player> = Vec::new();
    match Player::new(&mut players, 1, "Bhargav".to_string()){
        Ok(())=>println!("New Player creted"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::new(&mut players, 2, "Bhargavi".to_string()){
        Ok(())=>println!("New Player creted"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::new(&mut players, 3, "Vishal".to_string()){
        Ok(())=>println!("New Player creted"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::new(&mut players, 4, "Potti".to_string()){
        Ok(())=>println!("New Player creted"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::new(&mut players, 5, "Madhumita".to_string()){
        Ok(())=>println!("New Player creted"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::new(&mut players, 6, "Brian".to_string()){
        Ok(())=>println!("New Player creted"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::add_funds(&mut players, 1, 500){
        Ok(())=>println!("Amount added to player"),
        Err(msg)=>println!("{}",msg),
    }
    match Player::add_funds(&mut players, 2, 5001){
        Ok(())=>println!("Amount added to player"),
        Err(msg)=>println!("{}",msg),

    }
     match Player::add_funds(&mut players, 3, 1111){
        Ok(())=>println!("Amount added to player"),
        Err(msg)=>println!("{}",msg),

    }
     match Player::add_funds(&mut players, 4, 7901){
        Ok(())=>println!("Amount added to player"),
        Err(msg)=>println!("{}",msg),

    }
    match Player::add_funds(&mut players, 5, 701){
        Ok(())=>println!("Amount added to player"),
        Err(msg)=>println!("{}",msg),

    }

    match Player::add_funds(&mut players, 6, 7017){
        Ok(())=>println!("Amount added to player"),
        Err(msg)=>println!("{}",msg),

    }
  
  

    let mut lottery_1 = Lottery::new();
    Lottery::lottery_state(&lottery_1);
    match Lottery::add_player_lottery(&mut lottery_1, &mut players, 1) {
        Ok(())=>println!("Player added successfully"),
        Err(msg)=>println!("{}",msg),
        
    }
     match Lottery::add_player_lottery(&mut lottery_1, &mut players, 2) {
        Ok(())=>println!("Player added successfully"),
        Err(msg)=>println!("{}",msg),
        
    }
     match Lottery::add_player_lottery(&mut lottery_1, &mut players, 3) {
        Ok(())=>println!("Player added successfully"),
        Err(msg)=>println!("{}",msg),
        
    }
     match Lottery::add_player_lottery(&mut lottery_1, &mut players, 4) {
        Ok(())=>println!("Player added successfully"),
        Err(msg)=>println!("{}",msg),
        
    }
    match Lottery::add_player_lottery(&mut lottery_1, &mut players, 5) {
        Ok(())=>println!("Player added successfully"),
        Err(msg)=>println!("{}",msg),
        
    }
    match Lottery::add_player_lottery(&mut lottery_1, &mut players, 6) {
        Ok(())=>println!("Player added successfully"),
        Err(msg)=>println!("{}",msg),
        
    }


    Lottery::lottery_state(&lottery_1);
    
    match Lottery::start_drawing(&mut lottery_1) {
        Ok(())=>println!("Drawing Started"),
        Err(msg)=>println!("{}",msg),
        
    }
  
    Lottery::lottery_state(&lottery_1);
    

    match Lottery::start_drawing(&mut lottery_1) {
        Ok(())=>println!("Drawing Started"),
        Err(msg)=>println!("{}",msg),
        
    }

    match Lottery::close_lottery(&mut lottery_1) {
        Ok(())=>println!("Lottery Successfully closed"),
        Err(msg)=>println!("{}",msg),
        
    }

    Lottery::lottery_state(&lottery_1);
    Lottery::select_winner(&mut lottery_1, &mut players);

    Player::print_players(&players);
    Lottery::lottery_state(&lottery_1);
    
    
    

   
    



    


}