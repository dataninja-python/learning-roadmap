use std::io;
use rand::Rng;
use std::cmp::Ordering;
// source: https://www.codegrepper.com/code-examples/rust/rust+u32+to+f64

#[derive(Debug)]
enum IncludeOrExclude {
    Inclusive,
    Exclusive,
}

#[derive(Debug)]
struct SetUp {
    beg: u32,
    end: u32,
    opt: IncludeOrExclude,
}

fn main () {
    // let x = 5;
    // let y = 10;
    // print!("x = {} and y = {}", x, y);
    let state = SetUp { beg: 1, end: 100, opt: IncludeOrExclude::Inclusive };
    let state_alt = SetUp { beg: 1, end: 100, opt: IncludeOrExclude::Exclusive };
    println!("{:#?}", state_alt);
    
    // let beg = 1;
    // let end = 2;
    // let range_opt = IncludeOrExclude::Inclusive;
    let secret_num = init(&state.beg, &state.end, &state.opt);
    println!("Secret Number: {}", secret_num);

    play_game(&secret_num)

    // let guess = get_guess();
    // println!("You guessed: {guess}");
}

fn init(low:&u32, high:&u32, range_cmd:&IncludeOrExclude) -> u32 {
    // get a secret number from the start to end range
    // examples of 2 methods:
    // 1. 1..=100 -> select a number from 1 to 100 (inclusive)
    // 2. 1..100 -> select a number from 1 to 100 (exclusive)
    match *range_cmd {
        IncludeOrExclude::Inclusive => rand::thread_rng().gen_range(*low..=*high),
        IncludeOrExclude::Exclusive => rand::thread_rng().gen_range(*low..*high),
        // _ => panic!("not told if random number should be inclusive or exclusive"),
    }
}

fn get_guess () -> u32 {
    println!("Guess the number!");
    println!("Please input your guess...");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input: u32 = match user_input.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Enter a number!"),
    };
    user_input
}

fn play_game (win_cond: &u32) {
    // PROGRAM
    // 1. initialize game = get a secret number
    // 2. start game loop = get player's guess
    // 3. compare guess to secret number
    // 4. if match, run win function
    // 5. if not match, continue game loop
    // let to_win = String::from(win_cond.to_string());

    loop {
        let guess = get_guess();
        println!("You guessed: {guess}");
        match guess.cmp(win_cond) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
