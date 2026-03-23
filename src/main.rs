#[derive(Debug)]
struct Player{
    id:u32,
    name:String,
    balance:u64,
}

impl Player{
    fn add_new_player(id:u32,name:String)->Player{
        Player { id:id, name:name, balance: 0 }
    }

    fn add_balance(&mut self,amount:u64){
        self.balance += amount
    }
}

enum LotteryState {
    Open,
    Drawing,
    Close,
}

struct Lottery {
    state:LotteryState,
    pool:u64,
    players:Vec<Player>
    
}

impl Lottery{
    fn open_new_lottery()->Lottery{
        Lottery {state:LotteryState::Open, pool:0,players:Vec::new()}
    }

    fn add_player(&mut self, mut players:Player)->Result<(),String>{
        if players.balance<50{
            return Err("Not enough balance".to_string());
        }
        players.balance-=50;
        self.pool+=50;
        self.players.push(players);
        return Ok(());
    }

    fn lottery_state(&self){
        println!("Total number of players joined: {}",self.players.len());
        for player in self.players.iter(){
            println!("Player: {}",player.balance);
        }
        println!("Total lottery pool: {}",self.pool);
    }
}



fn main(){

    let mut player_1 = Player::add_new_player(1,"Bhargav".to_string());
    let mut player_2 = Player::add_new_player(1, "Potti".to_string());
    player_1.add_balance(100);
    player_2.add_balance(100);
    let mut lottery_1 = Lottery::open_new_lottery();
    match lottery_1.add_player(player_1){
        Ok(())=>println!("Player added"),
        Err(msg)=>println!("{}",msg),
    }
    match lottery_1.add_player(player_2){
        Ok(())=>print!("Player added"),
        Err(msg)=>println!("{}",msg),
    }
    lottery_1.lottery_state();

    
  
   
    

    
}