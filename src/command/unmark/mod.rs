use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute, terminal, cursor
};
use crossterm::terminal::ClearType;

pub fn start_interactive_session(file_path: &str) -> std::io::Result<()> {
    execute!(
        std::io::stdout(),
        terminal::EnterAlternateScreen,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
    )
    .unwrap();


    println!("Interactive session for 'unmark' command. Press 'Ctrl+C' to quit.");
    let contents = crate::command::marks::read_marks(file_path).unwrap();
    println!("{}", contents);

    loop {
        println!("Back to the top \n");
        let stuff = event::read().unwrap();
        // println!("{:#?} \n", stuff);
        match event::read().unwrap() {
            event::Event::Key(KeyEvent {
                                  code,
                                  kind: _,
                                  modifiers, state
                              }) => {
                if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('d') {
                    // Handle Ctrl+C to exit
                    println!("DONE");

                    break;
                }

                match code {
                    // KeyCode::Enter => {
                    //     // Handle Enter key
                    //     println!("Executing command...");
                    // }
                    KeyCode::Char('d') => {
                        // Example: Detect 'exit' command to quit the session
                        println!("Exiting interactive session.");
                        break;
                    }
                    _ => {
                        // Handle other key events
                        println!("You pressed: {:?}", code);
                    }
                }
            }
            _ => {}
        }
    }

    println!("DONE");
    return Ok(());

}
