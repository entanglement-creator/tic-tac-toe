use std::{
    cmp::min,
    fs::File, 
    io::{stdin, stdout, Read, Stdin, Stdout, Write}
};

use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TicTacCharacter {
    Tic,
    Tac,
    Null
}

impl TicTacCharacter {
    fn anti(&self) -> Self {
        match self {
            Self::Tic => Self::Tac,
            Self::Tac => Self::Tic,
            Self::Null => Self::Null
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Tic => 'X',
            Self::Tac => 'O',
            Self::Null => ' '
        }
    }
}

impl From<u8> for TicTacCharacter {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Null,
            1 => Self::Tic,
            2 => Self::Tac,
            _ => panic!()
        }
    }
}

impl Default for TicTacCharacter {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Table {
    table: Vec<Vec<TicTacCharacter>>
}

impl Table {
    fn view(&self, output: &mut Stdout) {
        let S = self.table.len();
        let mut result = String::with_capacity(S * (16 * S + 2));

        let index_cell_str = |index: usize| -> String {
            let index = index.to_string();
            let index = index.as_str();
            let mut result = String::with_capacity(3);
            result.push_str("0".repeat(3 - index.len()).as_str());
            result.push_str(index);
            result
        };

        for i in 0..(S - 1) {
            //Line 1
            for j in 0..(S - 1) {
                result.push_str(" ".repeat(5).as_str());
                result.push('|');
            }

            result.push_str(" ".repeat(5).as_str());
            result.push('\n');
            //Line 2
            for j in 0..(S - 1) {
                if self.table[i][j] == TicTacCharacter::Null {
                    result.push(' ');
                    result.push_str(index_cell_str(S * i + j + 1).as_str());
                    result.push_str(" |");
                } else {
                    result.push_str("  ");
                    result.push(self.table[i][j].to_char());
                    result.push_str("  |");
                }
            }

            if self.table[i][S - 1] == TicTacCharacter::Null {
                result.push(' ');
                result.push_str(index_cell_str(S * (i + 1)).as_str());
                result.push_str(" \n");
            } else {
                result.push_str("  ");
                result.push(self.table[i][S - 1].to_char());
                result.push_str("  \n");
            }
            //Line 3
            for j in 0..(S - 1) {
                result.push_str("_".repeat(5).as_str());
                result.push('|');
            }

            result.push_str("_".repeat(5).as_str());
            result.push('\n');
        }

        //Line 1
        for j in 0..(S - 1) {
            result.push_str(" ".repeat(5).as_str());
            result.push('|');
        }

        result.push_str(" ".repeat(5).as_str());
        result.push('\n');
        //Line 2
        for j in 0..(S - 1) {
            if self.table[S - 1][j] == TicTacCharacter::Null {
                result.push(' ');
                result.push_str(index_cell_str(S * (S - 1) + j + 1).as_str());
                result.push_str(" |");
            } else {
                result.push_str("  ");
                result.push(self.table[S - 1][j].to_char());
                result.push_str("  |");
            }
        }

        if self.table[S - 1][S - 1] == TicTacCharacter::Null {
            result.push(' ');
            result.push_str(index_cell_str(S * S).as_str());
            result.push_str(" \n");
        } else {
            result.push_str("  ");
            result.push(self.table[S - 1][S - 1].to_char());
            result.push_str("  \n");
        }
        //Line 3
        for j in 0..(S - 1) {
            result.push_str(" ".repeat(5).as_str());
            result.push('|');
        }

        result.push_str(" ".repeat(5).as_str());
        result.push('\n');

        output.write(result.as_bytes());
        output.flush().unwrap();
    }

    fn check_win(&self, character: TicTacCharacter) -> bool {
        let S = self.table.len();

        //Horizontal
        for i in 0..S {
            let mut line = Vec::with_capacity(S);
            for j in 0..S {
                line.push(self.table[i][j]);
            }

            if !line.contains(&character.anti()) && !line.contains(&TicTacCharacter::Null) {
                return true;
            }
        }
        //Vertical
        for j in 0..S {
            let mut line = Vec::with_capacity(S);
            for i in 0..S {
                line.push(self.table[i][j]);
            }

            if !line.contains(&character.anti()) && !line.contains(&TicTacCharacter::Null) {
                return true;
            }
        }
        //Left Dioganal
        for i0 in 0..(S - 1) {
            let mut line = Vec::with_capacity(min(S, S) - i0);
            for i in 0..(min(S, S) - i0) {
                line.push(self.table[i0 + i][i]);
            }

            if !line.contains(&character.anti()) && !line.contains(&TicTacCharacter::Null) {
                return true;
            }
        }
        
        //Right Dioganal
        for i0 in 0..(S - 1) {
            let mut line = Vec::with_capacity(min(S, S) - i0);
            for i in 0..(min(S, S) - i0) {
                line.push(self.table[i0 + i][S - i - 1]);
            }

            if !line.contains(&character.anti()) && !line.contains(&TicTacCharacter::Null) {
                return true;
            }
        }
        //Not Win
        return false;

    }

    fn create_with(size: usize) -> Self {
        let mut table: Vec<Vec<TicTacCharacter>> = Vec::with_capacity(size * size);
        for _ in 0..size {
            let mut line = Vec::with_capacity(size);
            for _ in 0..size {
                line.push(TicTacCharacter::Null);
            }
            table.push(line);
        }

        Self {
            table: table
        }

    }

    fn insert(&mut self, index: usize, character: TicTacCharacter) -> Result<(), String> {
        let S = self.table.len();

        if index >= S * S {
            return Err(String::from("A place which you want insert your character is located out the table.\n"));
        }

        for y in 0..S {
            for x in 0..S {
                if index == S * y + x {
                    let value_position = self.table[y][x];
                    if value_position == TicTacCharacter::Null {
                        self.table[y][x] = character;
                    } else {
                        return Err(String::from("In a place which you want insert your character yet inserted a character.\n"))
                    }
                }
            }
        }

        Ok(())
    }

    fn filed(&self) -> bool {
        let mut filed = true;

        for i in 0..self.table.len() {
            filed = filed && !self.table[i].contains(&TicTacCharacter::Null);
        }

        filed
    }
}

impl From<Vec<Vec<u8>>> for Table {
    fn from(value: Vec<Vec<u8>>) -> Self {
        let mut table: Vec<Vec<TicTacCharacter>> = Vec::with_capacity(value.capacity());
        for i in value {
            let mut line = Vec::with_capacity(i.capacity());
            for j in i {
                line.push(TicTacCharacter::from(j));
            }
            table.push(line);
        }
        let table = table;

        Self {
            table: table
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Game {
    names_palyers: (String, String),
    table: Table,
    motions_character: TicTacCharacter
}

#[derive(Clone, Copy)]
enum MotionResult {
    Win,
    Continue
}

impl Game {
    fn set_as(&mut self, game: Game) {
        self.names_palyers = game.names_palyers;
        self.table = game.table;
        self.motions_character = game.motions_character;
    }

    fn start_new_game(&mut self, output: &mut Stdout, input: &Stdin) {
        output.write("A name of 1th player: ".as_bytes());
        output.flush().unwrap();
        let mut name = String::new();
        input.read_line(&mut name);
        self.names_palyers.0 = name.trim().to_string();

        output.write("A name of 2th player: ".as_bytes());
        output.flush().unwrap();
        let mut name = String::new();
        input.read_line(&mut name);
        self.names_palyers.1 = name.trim().to_string();

        let mut size: usize = 0;

        loop {
            output.write("A size of table: ".as_bytes());
            output.flush().unwrap();
            let mut string_size = String::new();
            input.read_line(&mut string_size);
            match string_size.trim().parse::<usize>() {
                Ok(value) => {
                    if value < 3 {
                        output.write("Try again, size of your game table must be great then 2.\n".as_bytes());
                        output.flush().unwrap();
                        continue;
                    }

                    if value > 10 {
                        output.write("Try again, size of your game table must be less then 11.\n".as_bytes());
                        output.flush().unwrap();
                        continue;
                    }

                    size = value;
                    break;
                },
                Err(_) => {
                    output.write("Try again, you input not number.\n".as_bytes());
                    output.flush().unwrap();
                },
            }
        }

        self.table = Table::create_with(size);
        self.motions_character = TicTacCharacter::Tic;
    }

    fn run(&mut self, output: &mut Stdout, input: &Stdin) {
        match File::open("last_game.json") {
            Ok(mut file) => {
                loop {
                    output.write("Do you want load last game? (yes|no): ".as_bytes());
                    output.flush().unwrap();

                    let mut choice = String::new();
                    input.read_line(&mut choice);
                    let choice = choice.trim();

                    match choice {
                        "Yes" | "YES" | "yes" | "Y" | "y" => {
                            let mut json = String::new();
                            file.read_to_string(&mut json);
                            let game: Game = from_str(json.as_str()).unwrap();
                            self.set_as(game);
                            break;
                        },
                        "No" | "NO" | "no" | "N" | "n" => {
                            self.start_new_game(output, input);
                            break;
                            
                        },
                        _ => {
                            output.write("Try again, you input not choice.\n".as_bytes());
                            output.flush().unwrap();
                        }
                    }
                }
                
            },
            Err(_) => self.start_new_game(output, input)
        }
        

        //Game Loop
        loop {
            output.write(format!("{} (X) - {} (O)\n", self.names_palyers.0, self.names_palyers.1).as_bytes());
            self.table.view(output);

            if self.motions_character == TicTacCharacter::Tic {
                output.write(format!("Current motion for {}.\n", self.names_palyers.0).as_bytes());
                output.flush().unwrap();
            } else {
                output.write(format!("Current motion for {}.\n", self.names_palyers.1).as_bytes());
                output.flush().unwrap();
            }

            output.write("E exit S save & exit M motion\n".as_bytes());
            output.write("> ".as_bytes());
            output.flush().unwrap();

            let mut choice = String::new();
            input.read_line(&mut choice);
            let choice = choice.trim();

            match choice {
                "E" | "e" => {
                    return;
                }

                "S" | "s" => {
                    let mut file = File::create("last_game.json").unwrap();

                    let json = to_string(self).unwrap();
                    let mut json = json.as_bytes();
                    file.write_all(&mut json);

                    return;
                }

                "M" | "m" => {
                    match self.motion(output, input) {
                        MotionResult::Continue => continue,
                        MotionResult::Win => {
                            loop {
                                output.write("Do you want start yet new game? (yes|no): ".as_bytes());
                                output.flush().unwrap();

                                let mut choice = String::new();
                                input.read_line(&mut choice);
                                let choice = choice.trim();

                                match choice {
                                    "No" | "NO" | "no" | "N" | "n" => return,
                                    "Yes" | "YES" | "yes" | "Y" | "y" => {
                                        self.start_new_game(output, input);
                                        break;
                                    },
                                    _ => {
                                        output.write("Try again, you input not choice.\n".as_bytes());
                                        output.flush().unwrap();
                                    }
                                }
                            }
                        }
                    }
                }

                _ => {
                    output.write("Try again, you input not choice.\n".as_bytes());
                    output.flush().unwrap();
                }
            }
        }
    }

    fn motion(&mut self, output: &mut Stdout, input: &Stdin) -> MotionResult {
        loop {
            let mut number: usize = 0;

            loop {
                if self.motions_character == TicTacCharacter::Tic {
                    output.write(format!("{}, enter a number position: ", self.names_palyers.0).as_bytes());
                } else {
                    output.write(format!("{}, enter a number position: ", self.names_palyers.1).as_bytes());
                }
                
                output.flush().unwrap();
                let mut number_string = String::new();
                input.read_line(&mut number_string);
                match number_string.trim().parse::<usize>() {
                    Ok(value) => {
                        number = value;
                        break;
                    },
                    Err(_) => {
                        output.write("Try again, you input not number.\n".as_bytes());
                        output.flush().unwrap();
                    },
                }
            }

            match self.table.insert(number - 1, self.motions_character) {
                Ok(_) => {
                    if self.table.check_win(self.motions_character) {
                        if self.motions_character == TicTacCharacter::Tic {
                            output.write(format!("{} win!\n", self.names_palyers.0).as_bytes());
                            self.table.view(output);
                        } else {
                            output.write(format!("{} win!\n", self.names_palyers.1).as_bytes());
                            self.table.view(output);
                        }

                        output.flush().unwrap();
                        return MotionResult::Win;
                    }

                    if self.table.filed() {
                        output.write("Null is winner!\n".as_bytes());
                        self.table.view(output);
                        output.flush().unwrap();
                        return MotionResult::Win;
                    }

                    self.motions_character = self.motions_character.anti();
                    return MotionResult::Continue;
                },
                Err(err) => {
                    output.write(err.as_bytes());
                    output.write("Try again.\n".as_bytes());
                    output.flush().unwrap();
                },
            }
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.run(&mut stdout(), &stdin());
}