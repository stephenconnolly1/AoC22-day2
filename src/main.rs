use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut total_score = 0;
        for line in lines {
            if let Ok(round) = line {
                // println!("{}", round);
                total_score += play_round(round);
            }
        }
        println!("Total score: {}", total_score);
    } else {
        println!("Unable to open file");
    }
}


fn play_round (round: String) -> i32 {
    let mut result = 0;
    match round.as_str() {
        /* 
        "A X" => result = 1+3,
        "A Y" => result = 2+6,
        "A Z" => result = 3+0,
        "B X" => result = 1+0,
        "B Y" => result = 2+3,
        "B Z" => result = 3+6,
        "C X" => result = 1+6,
        "C Y" => result = 2+0,
        "C Z" => result = 3+3,
        */
        "A X" => /*lose, scissors */ result = 0+3,
        "A Y" => /*draw, rock     */ result = 3+1,
        "A Z" => /*win, paper     */ result = 6+2,
        "B X" => /*lose, rock     */ result = 0+1,
        "B Y" => /*draw, paper    */ result = 3+2,
        "B Z" => /*win, scissors  */ result = 6+3,
        "C X" => /*lose, paper    */ result = 0+2,
        "C Y" => /*draw, scissors */ result = 3+3,
        "C Z" => /*win, rock      */ result = 6+1,
        _ => println!("Unexpected Round: {}", round),
    }
    println!("{round} : {result}");
    result
}




// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}