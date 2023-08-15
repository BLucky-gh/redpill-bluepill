use clap::Parser;
use rand::random;
use strum::IntoEnumIterator;

fn main() {
    let args = Args::parse();

    let mut players: Vec<Player> = (0..args.guessers)
        .map(|_| Player::new(PlayerType::Guesser))
        .chain((0..args.reds).map(|_| Player::new(PlayerType::AlwaysRed)))
        .chain((0..args.blues).map(|_| Player::new(PlayerType::AlwaysBlue)))
        .collect();

    for i in 1..=args.iter_count {
        println!("Round {i}");
        println!(
            "There are currently {} players, of which there are",
            players.len()
        );
        for kind in PlayerType::iter() {
            println!(
                "{} are {kind}",
                players.iter().filter(|player| player.kind == kind).count()
            )
        }

        let before_count = players.len();
        players.iter_mut().for_each(Player::choose_pill);
        let reds = players
            .iter()
            .filter(|player| player.vote == Some(Pill::Red))
            .count();
        let blues = players
            .iter()
            .filter(|player| player.vote == Some(Pill::Blue))
            .count();
        if reds > blues {
            players.retain(|player| player.vote == Some(Pill::Red))
        }
        players.iter_mut().for_each(Player::reset_vote);
        println!(
            "{} players died, {} survive",
            before_count - players.len(),
            players.len()
        );
        println!();
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'g', default_value_t = 0, long)]
    guessers: u32,
    #[arg(short = 'r', default_value_t = 0, long)]
    reds: u32,
    #[arg(short = 'b', default_value_t = 0, long)]
    blues: u32,
    #[arg(default_value_t = 10, long)]
    iter_count: u32,
}

#[derive(Debug, PartialEq, Eq, strum_macros::EnumIter, strum_macros::Display)]
enum Pill {
    Blue,
    Red,
}
#[derive(Debug, PartialEq, Eq, strum_macros::EnumIter, strum_macros::Display)]
enum PlayerType {
    Guesser,
    AlwaysRed,
    AlwaysBlue,
}

#[derive(Debug)]
struct Player {
    kind: PlayerType,
    pub vote: Option<Pill>,
}
impl Player {
    fn new(kind: PlayerType) -> Self {
        Self { vote: None, kind }
    }
    fn choose_pill(&mut self) {
        self.vote = Some(match self.kind {
            PlayerType::Guesser => {
                if random::<bool>() {
                    Pill::Blue
                } else {
                    Pill::Red
                }
            }
            PlayerType::AlwaysBlue => Pill::Blue,
            PlayerType::AlwaysRed => Pill::Red,
        });
    }
    fn reset_vote(&mut self) {
        self.vote = None
    }
}
