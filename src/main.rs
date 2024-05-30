use std::{ops::Range, process::exit};

use ::api_calls::api_calls::{ del_blackboards, del_blackboards_specific, get_blackboards, get_blackboards_status, post_blackboards, post_blackboards_clear };
use colored::Colorize;

use reedline::{
    default_emacs_keybindings,
    ColumnarMenu,
    DefaultCompleter,
    DefaultPrompt,
    Emacs,
    KeyCode,
    KeyModifiers,
    MenuBuilder,
    Reedline,
    ReedlineEvent,
    ReedlineMenu,
    Signal,
};

use regex::Regex;

fn get_args(pat: &str) -> Vec<&str> {
    let re_space = Regex::new(r" ").unwrap();
    let coll: Vec<&str> = re_space.split(pat.trim()).collect();
    return coll;
}

fn main() {
    let commands: Vec<String> = vec![
        "clear".to_string(),
        "create".to_string(),
        "validate".to_string(),
        "delete".to_string(),
        "list".to_string(),
        "help".to_string(),
        "exit".to_string(),
        "set".to_string(),
        "get".to_string()
    ];

    let mut address = "127.0.0.1:5000".to_string();

    let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.to_vec(), 2));

    // Use the interactive menu to select options from the completer
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    // Set up the required keybindings
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(
            vec![ReedlineEvent::Menu("completion_menu".to_string()), ReedlineEvent::MenuNext]
        )
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut line_editor = Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode);

    let crabking = "Crabking".bright_white().bold();

    let prompt = DefaultPrompt {
        left_prompt: reedline::DefaultPromptSegment::Basic(crabking.to_string()),
        right_prompt: reedline::DefaultPromptSegment::Empty,
    };

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let mut args = get_args(&buffer);
                let com = args[0];
                args.remove(0);
                success(com, args, &mut address, &commands);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}

fn success(buffer: &str, args: Vec<&str>, address: &mut String, commands: &Vec<String>) {
    match buffer {
        "set" => {
            if args.len() > 1 {
                to_many_args_error();
                return;
            }
            if args.len() == 0 {
                to_few_args_error();
                return;
            }
            // this is easier then keeping a fixed length string
            let _ = address.replace(".*", args[0]);
        },
        "get" => {
            println!("Current Address is {}", address);
        },
        "create" => {
            if check_args(args.clone(), 2..3) {
                let res = post_blackboards(args[0].to_string(), args[1].to_string().parse::<u32>().unwrap_or(100));
                handle_simple_response(res);
            }
        },
        "delete" => {
            if check_args(args.clone(), 1..3) {
                let res;
                if args.len() == 1 {
                    res = del_blackboards_specific(args[1].to_string());
                } else {
                    res = del_blackboards();
                }
                handle_simple_response(res);
            }
        },
        "validate" => {
            if check_args(args.clone(), 1..2) {
                let res = get_blackboards_status(args[1].to_string());
                handle_simple_response(res);
            }
        }
        "clear" => {
            if check_args(args.clone(), 1..2) {
                let res = post_blackboards_clear(args[0].to_string());
                handle_simple_response(res);
            }
        },
        "list" => {
            if check_args(args.clone(), 0..1) {
                let res = get_blackboards();
                handle_simple_response(res);
            }
        },
        "help" => {
            for name in commands.iter() {
                match name.as_str() {
                    "set" => println!("Usage: {} <ip address>. Set the ip address", name),
                    "get" => println!("Usage: {}. Get the currently set ip address", name),
                    "create" => println!("Usage: {} <name> <duration>. If duration is not parsable, defaults to 100.", name),
                    "delete" => println!("Usage: {} (<name>). Delete all boards, or optionally a specified one.", name),
                    "list" => println!("Usage: {}. List all boards.", name),
                    "validate" => println!("Usage: {}. Validate a board.", name),
                    "exit" => println!("Usage: {}. Exit", name),
                    "clear" => println!("Usage: {}. Clear board.", name),
                    &_ => println!("Usage: {} <arg>", name),
                }
            }
        },
        "exit" => {
            println!("Goodbye");
            exit(0);
        }
        &_ => { println!("{}", "invalid command".red()) }
    }
}

fn to_many_args_error() {
    println!("{}", "Too many args".red().bold())
}

fn to_few_args_error() {
    println!("{}", "Expected Argument(s)".red().bold())
}

fn check_args(args: Vec<&str>, num_of_accepted_args: Range<usize>) -> bool {
    if args.len() > num_of_accepted_args.clone().min().unwrap() {
        to_many_args_error();
        return false;
    }
    if args.len() < num_of_accepted_args.max().unwrap() {
        to_few_args_error();
        return false;
    }
   return true; 
}

// Generic response handler. Since all api calls basicly just print the result, we can just use a generic function to express this
fn handle_simple_response(response: Result<reqwest::blocking::Response, reqwest::Error>) {
    if response.is_err() {
        println!("Connection error occured. Make sure your IP address is correct");
        return;
    }
    let res = response.unwrap();
    let status = res.status();
    let text = res.text().unwrap();
    if status.is_success() {
        println!("{}", text)
    } else {
        println!("{0}: {1}", status.as_str().yellow().italic(), text.white())
    }
}

fn ok(_: String) -> String {
    return "Ok".green().to_string();
}
