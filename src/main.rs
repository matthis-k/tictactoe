use serde_scan::scan;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Copy)]
enum Player {
    #[default]
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Player::X => "X",
            Player::O => "O",
        })
    }
}

#[derive(Default, Debug, Clone)]
struct Board {
    board: [[Option<Player>; 3]; 3],
}
impl Board {
    fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get(x.try_into().unwrap(), y.try_into().unwrap())
            .is_none()
    }

    fn get(&self, x: usize, y: usize) -> Option<Player> {
        if let Some(line) = self.board.get(y) {
            if let Some(field) = line.get(x) {
                field.clone()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn place(&mut self, p: Player, m: &Move) {
        let Move(x, y) = m;
        self.board[*y as usize][*x as usize] = Some(p);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.board.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                match field {
                    Some(field) => field.fmt(f),
                    None => f.write_str(" "),
                }?;
                f.write_str(if x < line.len() - 1 { "|" } else { "\n" })?;
            }
            f.write_str(if y < line.len() - 1 { "-+-+-\n" } else { "" })?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
struct Game {
    turn: Player,
    board: Board,
}

impl Game {
    fn make_move(&mut self, m: &Move) -> Result<(), InvalidMove> {
        if !self.is_legal(m) {
            return Err(InvalidMove);
        }

        self.board.place(self.turn.clone(), &m);
        self.turn = match self.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(())
    }
    fn is_legal(&self, m: &Move) -> bool {
        self.board.is_empty(m.0, m.1)
    }
    fn winner(&self) -> Option<Player> {
        let cols = (0..3).map(|row| (0..3).map(move |col| (row, col)).collect::<Vec<_>>());
        let rows = (0..3).map(|col| (0..3).map(move |row| (row, col)).collect::<Vec<_>>());
        let diags = [
            (0..3).map(|i| (i, i)).collect::<Vec<_>>(),
            (0..3).map(|i| (i, 2 - i)).collect::<Vec<_>>(),
        ];
        let all: Vec<_> = rows.chain(cols).chain(diags).collect();
        for win_set in all {
            let occupation: Vec<_> = win_set
                .iter()
                .flat_map(|&(x, y)| self.board.get(x, y))
                .collect();
            if occupation.len() != 3 {
                continue;
            }
            if occupation.iter().all(|e| e == occupation.first().unwrap()) {
                return occupation.first().map(Clone::clone);
            }
        }
        None
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)
    }
}

#[derive(Debug)]
struct InvalidMove;
#[derive(Debug)]
struct InvalidMoveFormat;

#[derive(Default, Debug, Clone)]
struct Move(i32, i32);

impl FromStr for Move {
    type Err = InvalidMoveFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok((x, y)) = scan!("{} {}\n" <- s) else {
            return Err(InvalidMoveFormat);
        };

        if !(1..=3).contains(&x) || !(1..=3).contains(&y) {
            Err(InvalidMoveFormat)
        } else {
            Ok(Self(x - 1, y - 1))
        }
    }
}

fn main() {
    let mut game = Game::default();
    while game.winner().is_none() {
        println!("{game}");
        println!("{} turn:", game.turn);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(m) = Move::from_str(input.as_str()) {
            match game.make_move(&m) {
                Ok(_) => {}
                Err(_) => {
                    println!("Invalid move, go again")
                }
            }
        } else {
            println!("wrong input format")
        }
    }
    println!("{game}");
    println!("{} won!!!", game.winner().unwrap());
}
