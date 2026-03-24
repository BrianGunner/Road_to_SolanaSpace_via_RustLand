struct Player{
    id: u32,
    name: String,
    balance: u64,
}
#[derive(Debug)]
enum LotteryState{
    Open, 
    Drawing, 
    Closed
}

struct Lottery{
    state:LotteryState,
    pool:u64,
    player_ids:Vec<u32>,
    ticket_price:u64,
}

impl Lottery{
    fn new()->Lottery{
        Lottery {state: LotteryState::Open, pool: 0, player_ids: Vec::new(),ticket_price:100}
    }

    fn add_players_lottery(&mut self,players:&mut Vec<Player>,id:u32)->Result<(),String>{

        for lottery_id in self.player_ids.iter(){
            if *lottery_id==id{
                return Err("Player id already exists in lottery".to_string());
            }
        }

        for player in players.iter_mut(){
            if player.id == id{
                if player.balance<self.ticket_price{
                return Err("Insufficient Balance".to_string());
            }
                player.balance -= self.ticket_price;
                self.pool += self.ticket_price;
                self.player_ids.push(id);
                return Ok(());
            }
            
        }
        
            
        

        

        Err("Player id does not exist".to_string())

    }

    fn print_lottery_state(&self){
        println!("Lottery State: {:?}",self.state);
        println!("Lottery Pool Amount: {}",self.pool);
        println!("Player ids joined Lottery: ");
        for id in self.player_ids.iter(){
            print!("{},",id);
        }
    }
}


impl Player {

    fn new(players:&mut Vec<Player>,id:u32,name:String){
        players.push(Player {id:id , name:name, balance:0});
    }

    fn print_players(players:&Vec<Player>){
        for player in players.iter(){
            println!("Name: {}, Balance: {}, id: {}",player.name,player.balance,player.id);
        }
    }

    fn fund_players(players:&mut Vec<Player>,amount:u64,id:u32)->Result<(),String>{
        
        
        for player in players.iter_mut(){
            if player.id==id{
                player.balance += amount;
                return Ok(());
            }  
        }

        Err("Id not found".to_string())
    }
    
}

fn main(){

    let mut players:Vec<Player> = Vec::new();
    Player::new(&mut players, 1, "Bhargav".to_string());
    Player::new(&mut players, 2, "Potti".to_string());
    match Player::fund_players(&mut players, 100, 2){
        Ok(())=>println!("Balance added"),
        Err(msg)=>println!("{}",msg),
    }
    Player::print_players(&players);

    let mut lottery_1 = Lottery::new();

    match lottery_1.add_players_lottery(&mut players, 2){
        Ok(())=>println!("Valid player id found and added to lottery"),
        Err(msg)=>println!("{}",msg),
    }
    match lottery_1.add_players_lottery(&mut players, 2){
        Ok(())=>println!("Valid player id found and added to lottery"),
        Err(msg)=>println!("{}",msg),
    }
    match lottery_1.add_players_lottery(&mut players, 1){
        Ok(())=>println!("Valid player id found and added to lottery"),
        Err(msg)=>println!("{}",msg),
    }

    match Player::fund_players(&mut players, 100, 1){
        Ok(())=>println!("Balance Added"),
        Err(msg)=>println!("{}",msg),
    }

     match lottery_1.add_players_lottery(&mut players, 1){
        Ok(())=>println!("Valid player id found and added to lottery"),
        Err(msg)=>println!("{}",msg),
    }


  

    lottery_1.print_lottery_state();



    

}