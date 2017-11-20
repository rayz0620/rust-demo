enum Message {
    Quit,
    Move{x: u32, y:u32},
    Print{text: String}
}

fn main() {
    let msg1 = Message::Move{x:4, y:7};
    let msg2 = Message::Print{text: "Arrived".to_string()};
    let msg3 = Message::Quit;
    let msg4 = Message::Quit;

    act(msg1);
    act(msg2);
    act(msg3);

    quit(msg4);
}

fn act(msg: Message) {
    match msg {
        Message::Quit => println!("Exiting...", ),
        Message::Move{x, y} => println!("Moving to point [{}, {}]", x, y),
        Message::Print{text} => {
            println!("Print: {}", text)
        }
    }
}

fn quit(msg: Message) {
    if let Message::Quit = msg {
        println!("Quit!", )
    } else {
        println!("Not quitting", )
    }
}