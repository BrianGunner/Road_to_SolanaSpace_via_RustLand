

#[derive(PartialEq)]
enum LotteryState{
    Open,
    Drawing,
    Closed,
}

struct Lottery{
    state:LotteryState,
}

impl Lottery{
    fn check_state(&self)->&str{
        match self.state{
            LotteryState::Open=>"State Open",
            LotteryState::Drawing=>"Drawing Progress",
            LotteryState::Closed=>"Lottery Closed",
        }
    }

    fn start_drawing(&mut self)->Result<(),String>{
        if self.state==LotteryState::Open{
            self.state = LotteryState::Drawing;
            return Ok(());
        }
        if self.state == LotteryState::Closed{
            return Err("Lottery already closed, please try later".to_string());
        }
        return Err("Lottery already Drawing".to_string());
    }

    fn close(&mut self)->Result<(),String>{
        if self.state == LotteryState::Drawing{
            self.state = LotteryState::Closed;
            return Ok(());
        }
        if self.state == LotteryState::Open{
            return Err("Cannot close state when Open".to_string())
        }
        return Err("State already Closed".to_string());

    }
    fn open(&mut self)->Result<(),String>{
        if self.state==LotteryState::Closed{
            self.state = LotteryState::Open;
            return Ok(());
        }
        if self.state == LotteryState::Open{
            return Err("Lottery already Open".to_string());
        }
        Err("Cannot Open Lottery when in Match".to_string())
    }
}

fn main(){
    let mut lottery = Lottery{state:LotteryState::Open};
    let result = lottery.check_state();
    println!("{}",result);

    match lottery.start_drawing(){
        Ok(())=>println!("Drawing successful"),
        Err(value)=>println!("{}",value),
    }
    let result = lottery.check_state();
    println!("{}",result);

    match lottery.close(){
        Ok(())=>println!("State Closed"),
        Err(msg)=>println!("{}",msg),
    }

    match lottery.close(){
        Ok(())=>println!("State Closed"),
        Err(msg)=>println!("{}",msg),
    }

    match lottery.start_drawing(){
        Ok(())=>println!("Drawing successful"),
        Err(value)=>println!("{}",value),
    }

    match lottery.open(){
        Ok(())=>println!("Open successful"),
        Err(value)=>println!("{}",value),
    }

}