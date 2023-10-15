use rand::Rng;
use std::io;
struct Player {
    name: String,
    score: u32,
}
trait Printable {
    fn to_string(&self) -> String;
}
impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }
}

fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed ot read input");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        }
    }
}
fn collect_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut num_players;
    loop {
        num_players = collect_input::<u32>("How many players (>= 2)?");
        if num_players < 2 {
            println!("Invalid ❌ no.! Please try again!");
            continue;
        } else {
            break;
        }
    }
    for i in 1..=num_players {
        let name = collect_input::<String>(format!("Player {} name:", i).as_str());
        players.push(Player { name, score: 0 })
    }
    players
}
fn create_max_range(players: &Vec<Player>) -> u32 {
    players.len() as u32 * 50
}
fn generate_number2(max_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_range)
}

use reqwest::*;

#[tokio::main]
async fn generate_number(max_range: u32) -> Result<u32> {
    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = std::env::var("URL")
        .expect("URL var not found")
        .replace("{MAX}", &max_range.to_string());
    let body = reqwest::get(url).await?.text().await?;
    let val = body.trim().parse::<u32>().expect("Error in parsing");
    // println!("value={:?}", val);
    Ok(val)
}
fn collect_guesses_into_proximities(players: &Vec<Player>, max_range: u32) -> Vec<(String, u32)> {
    let mut players_proximities = Vec::<(String, u32)>::new();
    let target =
        generate_number(create_max_range(players)).expect("Failure in generating random value");
    for player in players {
        println!("{}'s turn", player.name);
        let guess = collect_input::<u32>(&format!("Guess the number (1-{max_range});"));
        players_proximities.push((player.name.clone(), guess.abs_diff(target)));
    }
    players_proximities
}
fn get_winner(player_proximities: &Vec<(String, u32)>) -> String {
    player_proximities[0].0.to_owned()
}
fn update_scores(players: &mut Vec<Player>, winner: &str) {
    for player in players {
        if player.name == winner {
            player.score += 1
        }
    }
}
fn print_scores(players: &Vec<Player>) {
    println!("Scores: 📊");
    for player in players {
        println!("- {}", player.to_string());
    }
}
fn play_game() {
    println!("Welcome to the Target Proximity Game! 🎮");
    let mut players = collect_players();
    let max_range = create_max_range(&players);
    loop {
        let mut player_proximities = collect_guesses_into_proximities(&players, max_range);
        player_proximities.sort_by_key(|&(_, v)| v);
        let winner = get_winner(&player_proximities);
        println!("Congratulations, {}! 🎉 You are the winner!", winner);
        update_scores(&mut players, &winner);
        print_scores(&players);
        let play_again: String = collect_input("Play again? (y/n) 🔄️");
        if play_again.to_ascii_lowercase() != "y" {
            break;
        }
    }
}
fn main() {
    // let s = String::
    // let p: Player = Player {
    //     name: "hi",
    //     score: 0,
    // };
    // let _ = generate_number(100);
    // println!("Hello, world! {}", x);
    play_game();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_max_range() {
        let players = vec![
            Player {
                name: "Bob".to_string(),
                score: 0,
            },
            Player {
                name: "Alice".to_string(),
                score: 0,
            },
        ];
        let max_range = create_max_range(&players);
        assert_eq!(max_range, 100);
    }
    #[test]
    fn test_valid_rng() {
        let max_val = 100;
        let rand_value = generate_number(max_val).unwrap();

        assert!(rand_value >= 1 && rand_value <= max_val);
    }
    #[test]
    fn test_player_to_string() {
        let player = Player {
            name: "Alice".to_string(),
            score: 3,
        };
        assert_eq!(player.to_string(), "Alice (3)");
    }
}
