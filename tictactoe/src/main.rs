use std::io;

mod tic_tac_toe {
    use std::collections::HashSet;

    pub struct Board {
        round: i32,
        state: Vec<Vec<char>>,
        winner: Option<u8>,
    }

    impl Board {
        pub fn new() -> Self {
            Board {
                round: 0,
                state: vec![
                    vec!['-', '-', '-'],
                    vec!['-', '-', '-'],
                    vec!['-', '-', '-'],
                ],
                winner: None,
            }
        }

        pub fn validate_input(point: &str) -> Result<Vec<u8>, String> {
            let coord = point.trim().split(",").collect::<Vec<&str>>();

            if coord.len() != 2 {
                return Err(
                    "Please provide exact 2 numbers, separated by comma (without spacing)"
                        .to_string(),
                );
            }

            coord.iter()
                .map(|c| {
                    let parsed: Result<u8, String> = c
                        .parse::<u8>()
                        .map_err(|e| format!("{}: {}", e.to_string(), c));
                    match parsed {
                        Ok(p) => {
                            if p > 9 || p < 1 {
                                return Err("Input digit must be within 1-9".to_string());
                            }
                            return Ok(p);
                        }
                        Err(e) => Err(e),
                    }
                })
                .collect::<Result<Vec<u8>, _>>()
        }

        pub fn convert_coord(v: &Vec<u8>) -> (usize, usize) {
            let x = usize::from(v[0] - 1);
            let y = usize::from(v[1] - 1);
            (x, y)
        }

        pub fn mark(&mut self, coord: (usize, usize)) {
            if let Some(_) = self.winner {
                return;
            }

            if self.is_marked(coord) {
                return;
            }

            let sign = if self.round % 2 == 0 { 'O' } else { 'X' };
            self.state[coord.0][coord.1] = sign;
            self.round += 1;
            self.iterate_board();
        }

        pub fn is_marked(&self, coord: (usize, usize)) -> bool {
            self.state[coord.0][coord.1] != '-'
        }

        fn iterate_board(&mut self) {
            for row in 0..3 as usize {
                let mut _row = self.state[0].clone();
                if let Some(_) = self.find_winner(&_row) {
                    return;
                }
                
                let mut _col = vec![];
                for col in 0..3 as usize {
                    _col.push(self.state[row][col]);
                }
                if let Some(_) = self.find_winner(&_col) {
                    return;
                }
            }

            let _00 = self.state[0][0];
            let _02 = self.state[0][2];
            let _11 = self.state[1][1];
            let _20 = self.state[2][0];
            let _22 = self.state[2][2];

            if let Some(_) = self.find_winner(&vec![_00, _11, _22]) {
                return;
            }
            self.find_winner(&vec![_02, _11, _20]);

        }

        fn find_winner(&mut self, vals: &Vec<char>) -> Option<()> {
            if HashSet::<&char>::from_iter(vals).len() == 1 && vals[0] != '-' {
                let winner = u8::from(if vals[0] == 'O' { 1 } else { 2 });
                self.winner = Some(winner);
                return Some(())
            }
            None            
        }

        pub fn winner(&self) -> Option<u8> {
            self.winner
        }

        pub fn output(&self) {
            println!(
                "Player {}'s turn: ",
                if self.round % 2 == 0 { 1 } else { 2 }
            );
            println!("------------------");
            for row in &self.state {
                let string: String = row.iter().collect();
                println!("{}", string);
            }
        }
    }
}

fn main() {
    use tic_tac_toe::Board;
    let mut board = Board::new();

    while let None = board.winner() {
        print!("{esc}c", esc = 27 as char); // clear screen

        let mut input = String::new();

        board.output();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result = Board::validate_input(&input);
        if let Err(e) = result {
            println!("{}", e);
            continue;
        }

        let v = result.unwrap();
        let coord = Board::convert_coord(&v);
        board.mark(coord);
    }
    println!("Player {} wins", board.winner().unwrap());
}
