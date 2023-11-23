use std::{io, sync::mpsc::{Sender, Receiver, self}, thread, str::FromStr};
use crate::prompt::event::{Command, EventType};
use self::event::{Event, CommandParseErr};

pub mod event;

// Encapsulates the view of the application, it is responsible for taking input 
// from the user, parsing it into a valid command, then send it as an eveto to a
// target channel where it should be and processed. After that it waits for the
// output of the command to be processed to print it out to the user.
pub struct Prompt {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl Prompt {
    pub fn new(sender: Sender<Event>) -> (Prompt, Sender<Event>) {
        // creating the prompt channel
        let (my_sender, my_receiver) = mpsc::channel::<Event>();
        let prompt = Prompt {
            sender: sender,
            receiver: my_receiver,
        };

        (prompt, my_sender)
    }

    pub fn start(self) {
        thread::spawn(move || {
            loop {
                println!("[Awaiting your command, Sir]:");
                let mut input = String::new();

                _ = io::stdin().read_line(&mut input);
                let input = input.trim();

                if input == "" {
                    // no command entered, continue without doing anything.
                    continue;
                }

                let result = parse_input(&input);

                if result.is_err() {
                    eprintln!("Sir.. `{}` is not recognized as a command! Please try again.", input);
                    eprintln!("For a list of available commands, just type `help` (without the single qoutes).");
                    eprintln!();
                    continue;
                }

                let (command, payload) = result.unwrap();

                let mut event = Event {
                    e_type: EventType::CommandInput,
                    command: command,
                    payload: payload,
                };

                // recieve command here
                if event.command == Command::Csv {

                    // append the first line
                    event.payload.push('\n');

                    loop { // read all next lines
                        let mut line = String::new();
                        _ = io::stdin().read_line(&mut line);
                        let line = line.trim();
                        if line == "" {
                            // end of Csv input reached.
                            break;
                        }
                        event.payload.push_str(line);
                        event.payload.push('\n');
                    }
                }

                // send event to the processor (the main)
                self.sender.send(event).expect("Problem while sending to channel");
        
                // recieve the output from the processor (the main)
                let output_event = self.receiver.recv().unwrap();

                // before printing the output, check if this is a term signal
                if output_event.command == Command::Exit {
                    break; // break to exit the thread without panic.
                }

                println!("");
                println!("");
                println!("{}", output_event.payload);
                println!("");
            }
        });
    }
}

fn parse_input(input: &str) -> Result<(Command, String), CommandParseErr> {
    let tokens: Vec<&str> = input.split(' ').collect();
    let command = Command::from_str(tokens[0]);
    if command.is_err() {
        return Err(command.err().unwrap());
    }

    let mut payload = String::new();
    if tokens.len() > 1 {
        payload = tokens[1..].join(" ");
    }

    Ok((command.unwrap(), payload))
}