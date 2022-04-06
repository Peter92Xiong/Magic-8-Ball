use colored::Colorize;
// use std::env;
use std::io::stdin;
use std::io::Write;
use rand::Rng; //must had this to cargo.toml and whatnot by cargo search rand
/**
 *
 *  Notes
 *
 * 1. Get problem statements
 * 2. design -- divide and conquer
 *          --divide and conquer
 *          -- ask interviwer questions
 *          -- ask yourself questions: ex. what are my inputs and outputs?
 * 3. write code
 * --------------------------------
 *
 *  Design
 *  Magic 8-ball program --loops forever
 * Done 1. Takes user inputted:
 *      if quit, exit
 *      if question, conthine
 * 2. create possible reponses
 * 2b. generate random number
 * 2c. pink response based on random number
 * 3. prints response
 * 4. loop
 *
 *
 */
fn main() {
    //TODO:
    //Vec(vec![response,color],vec![response,color])
    let mut responses = Vec::new();
    let response1 = vec!["Yes definitely.","g"];
    let response2 = vec!["Ask again later.","y"];
    let response3 = vec!["Don't count on it.","r"];
    responses.push(response1);
    responses.push(response2);
    responses.push(response3);

    //generate random number generator
    let mut rng = rand::thread_rng();
    let mut output;

    //1. take user input
    println!("Ask me a question.");
    loop {
        print!("{}", "Magic 8-ball> ".bright_yellow());
        match std::io::stdout().flush() {
            Ok(o) => o,
            Err(_e) => {println!("Error getting stdout:"); return;}
        }
        let mut input = String::new(); // can grow (dynamic)
        let stdinput = stdin(); //stander input
        stdinput
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        //println!("");
        if input.trim() == "quit" || input.trim() == "q" || input.trim() == "exit" || input.trim() == "e" {break;}

        //generate random number
        let random_index = rng.gen_range(0..responses.len());
        //use random number to pick a responses
        output = match responses.get(random_index){
            Some(o) => o,
            None => {println!("No responses avaible"); return;}
        };
        match output[1]{
            "g" =>println!("output: {}", output[0].bright_green()),
            "r" =>println!("output: {}", output[0].bright_red()),
            "y" =>println!("output: {}", output[0].bright_yellow()),
            _ => {println!("Invalid response color found"); return;}
        }
        // println!("{}", output[0]);


    }
}

// Graveyard
//     let args: Vec<String> = env:: args().collect();
//     //let testvev: Vec<String> = Vec:: new(); //dynamic
//     let mut testvec = Vec::new();
//     testvec.push("test");
//     testvec.push("test2");

//     let testvec2 = vec!["test", "test2"]; //static

//     let mut teststring = String::new(); //not s type string
//     teststring.push("Hello");
//     teststring.push("World");

//     let teststring2 = "hello world"; //this is static! even if you put mut

//     //or you can do
//     // let testvec = Vec::new();
//     // args[0] --> always the name of the executable that we just ran
//     // args[1+] --> arguments
//     if args.len() < 2 {return;}
//     if args[1].trim() == "green"{
//         println!("{}","Hello, world!".green());
//     }
//     else if args[1].trim() == "red"{
//         println!("{}","Hello, world!".red());
//     }
//     else{
//     println!("{}","Hello, world!".yellow());
//     }

// }
