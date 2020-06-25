use rand::Rng;
use std::io;
struct  Player{
    name:String,
    score:u16,
}
fn main() {
    let mut input = String::new();
    let no_of_players:u16;

    println!("Enter the Total Number of no_of_players: ");
    
    loop{
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let test = input.trim().parse::<u16>();
        match test {
            Ok(num) => {
                no_of_players=num;
                input.clear();
                break;
            },
            Err(e) => {
                println!("Please Enter a valid number! {}",e);
                input.clear();
            },
        }
    }
    let mut players = Vec::new();
    for index in 0..no_of_players {
        println!("Enter Name of Player: {} ",index);
        
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let player = Player{
            name:input.trim().to_string(),
            score:0
        };

        players.push(player);
        input.clear();
    }
    println!("\nGame is started.. Press enter for next turn!\n");
    let mut turn:u16=1;
    loop{
        for index in 0..no_of_players {
            let mut dice = rand::thread_rng().gen_range(1, 7);
               
                //let player =  &mut players[index as usize];
                let mut player = Player{
                    name:players[index as usize].name.clone(),
                    score:players[index as usize].score
                };

                if dice==6{
                    dice = rand::thread_rng().gen_range(1, 7);
                    if dice ==6{
                        dice = rand::thread_rng().gen_range(1, 7);
                        if dice ==6{
                            println!("{} got 3 sixex in a row!",player.name);
                            dice = 0;
                        }else{
                            dice+=12;
                        }
                    }else{
                        dice+=6;
                    }
                }
                if player.score+dice<=100{
                    players[index as usize].score+=dice;
                    player.score+=dice;
                }
            
            println!("Turn {} Dice Roll of Player {} - {} is {} and Total {}",turn,index+1,players[index as usize].name,dice,players[index as usize].score);
            if players[index as usize].score==100{
                println!("Congratulations! Player {} has won on turn {}",players[index as usize].name,turn);
                return;                
            }

            for other_player in &mut players {
                if other_player.score ==player.score && other_player.name!=player.name && other_player.score>0{
                    println!("Alas! {} has kicked out by {} at score of {}",other_player.name,player.name,other_player.score);
                    other_player.score=0;
                }
            }
        }
        io::stdin().read_line(&mut input).expect("Failed to read line");       
        turn+=1;
    }
}