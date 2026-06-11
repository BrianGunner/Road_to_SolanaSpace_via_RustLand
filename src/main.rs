

enum ProgramType{
    SystemProgram,
    TokenProgram,
    GameProgram,
}
struct Account{
    address:u32,
    lamports:u64,
    data:Vec<u8>,
    owner:ProgramType,

}
struct TokenProgram;
impl TokenProgram {
    fn serialize(token_data:&TokenAccountData)->Vec<u8>{
        let mut data: Vec<u8> = Vec::new();
        let balance_bytes = token_data.balance.to_le_bytes();
        for bytes in balance_bytes.iter(){
            data.push(*bytes);
        }
        let mut frozen_bytes = 0_u8;
        if token_data.frozen==true{
            frozen_bytes=1;
        }
        data.push(frozen_bytes);
        let version_bytes = token_data.version;
        data.push(version_bytes);
        return data;
    }
    fn deserialize(data_vector:&Vec<u8>)->TokenAccountData{
        let mut re_balance_bytes = [0u8;8];
        re_balance_bytes.copy_from_slice(&data_vector[0..8]);
        let re_balance = u64::from_le_bytes(re_balance_bytes);
        let frozen_bytes = data_vector[8];
        let mut re_frozen = false;
        if frozen_bytes==1{
            re_frozen=true;
        }
        let re_version = data_vector[9];
        return TokenAccountData { balance: re_balance, frozen: re_frozen, version: re_version };

    }
}

struct GameProgram;

impl GameProgram{
    fn deserialize(token_data_vec:&Vec<u8>)->GameAccountData{
        let mut xp_bytes = [0u8;8];
        xp_bytes.copy_from_slice(&token_data_vec[0..8]);
        let xp = u64::from_le_bytes(xp_bytes);
        let alive_bytes = token_data_vec[8];
        let mut alive = false;
        if alive_bytes==1{
            alive=true;
        }
        let level = token_data_vec[9];
        return GameAccountData { xp, alive, level }
    }
}

 #[derive(Debug)]
struct TokenAccountData{
    balance:u64,
    frozen:bool,
    version:u8,
}
 #[derive(Debug)]
struct GameAccountData{
    xp:u64,
    alive:bool,
    level:u8,
}

struct JackPotVault{
    total:u64,
    round_id:u64,
}

fn main(){

    let token_data_1 = TokenAccountData{balance:10000,frozen:false,version:1};
    println!("token_data_1: {:?}",token_data_1);
    let serialized_token_data = TokenProgram::serialize(&token_data_1);
    println!("token data serialized: {:?}",serialized_token_data);
    let de_serialized_token_data = TokenProgram::deserialize(&serialized_token_data);
    println!("token_data deserialized: {:?}",de_serialized_token_data);
    let de_serialized_game_data = GameProgram::deserialize(&serialized_token_data);
    println!("Deserialized game data: {:?}",de_serialized_game_data);
    let round_100 = JackPotVault{total:1000,round_id:100};
    let jackpot_account = Account{address:99,lamports:0,data:vec![],owner:ProgramType::GameProgram};


}

