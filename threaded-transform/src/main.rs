use std::io::Read;
use std::sync::mpsc;
use std::thread;
use std::fs::File;
use prompt::Prompt;
use prompt::event::Command;
use prompt::event::Event;
use prompt::event::EventType;
use transformers::transform;

// modules
mod transformers;
mod prompt;

/// Application's entry point.
fn main() {
    // creating channel for communication
    let (my_sender, my_receiver) = mpsc::channel::<Event>();
    
    // I give you my sender, you give me your sender and so we can exchange messages :)
    let (prompt, prompt_sender) = Prompt::new(my_sender);
    
    // let's start the fun ;)
    prompt.start();

    let handle = thread::spawn(move || { 
        loop {
            // receiving commands from prompt
            let mut event = my_receiver.recv().unwrap();

            // process the command
            if event.command == Command::Exit {
                eprintln!("Termination requested by the user, terminating...");
                eprintln!("Send termination signal to the prompt thread...");
                
                let term_event = Event {
                    e_type: EventType::CommandOutput,
                    command: Command::Exit,
                    payload: "Termination requested by the user".to_string()
                };
                _ = prompt_sender.send(term_event);
                eprintln!("Termiation signal sent!");
                break;
            }

            // check if input is needed from file
            if event.payload.starts_with("file:") {
                let (_, mut file_path) = event.payload.split_once(':').unwrap();
                file_path = file_path.trim();
                let mut file = File::open(file_path).expect("Unable to open the file");
                let mut content = String::new();
                file.read_to_string(&mut content).expect("Unable to read the file");
                
                // resetting the payload with the file content
                // todo: this is not optimal, I think we should provide something like stream here, but this is a future work anyway.
                event.payload = content;
            }

            // start transformation
            let result = transform(event.payload, event.command);
            
    
            let mut o_event = Event {
                e_type: EventType::CommandOutput,
                command: event.command,
                payload: String::new()
            };
    
            if result.is_ok() {
                o_event.payload = result.unwrap();
            } else {
                o_event.e_type = EventType::CommandError;
                o_event.payload = result.unwrap_err().to_string();
            }
    
            // send output back to the prompt
            prompt_sender.send(o_event).unwrap();
        }
    });

    handle.join().unwrap();
    eprintln!("Program terminated!");
}
