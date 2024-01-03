use inquire::{Confirm, MultiSelect};
use inquire::list_option::ListOption;
use inquire::validator::Validation;

// pub fn get_input_for_unmark() {
//     let _marks = crate::command::marks::read_marks("marks.txt").unwrap();
//     let marks = _marks.lines().collect::<Vec<&str>>();
//
//     let _selected_marks = MultiSelect::new("Select marks you no longer need", marks)
//         .with_formatter(&|marks| format!("{} marks selected", marks.len()))
//         .with_validator(&|marks: &[ListOption<&&str>]| {
//             match marks.len() >= 1 {
//                 true => Ok(Validation::Valid),
//                 false => Ok(Validation::Invalid("Select at least one mark".into())),
//             }
//         })
//         .prompt()
//         .unwrap();
//
//     let _confirm_unmark = Confirm::new("Proceed with un-marking?")
//         .with_default(false)
//         .prompt()
//         .unwrap();
// }

// pub fn start_interactive_session(file_path: &str) -> std::io::Result<()> {
//     execute!(
//         std::io::stdout(),
//         terminal::EnterAlternateScreen,
//         terminal::Clear(ClearType::All),
//         cursor::MoveTo(0, 0),
//     )
//     .unwrap();
//
//
//     println!("Interactive session for 'unmark' command. Press 'Ctrl+C' to quit.");
//     let contents = crate::command::marks::read_marks(file_path).unwrap();
//     println!("{}", contents);
//
//     loop {
//         println!("Back to the top \n");
//         let stuff = event::read().unwrap();
//         // println!("{:#?} \n", stuff);
//         match event::read().unwrap() {
//             event::Event::Key(KeyEvent {
//                                   code,
//                                   kind: _,
//                                   modifiers, state
//                               }) => {
//                 if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('d') {
//                     // Handle Ctrl+C to exit
//                     println!("DONE");
//
//                     break;
//                 }
//
//                 match code {
//                     // KeyCode::Enter => {
//                     //     // Handle Enter key
//                     //     println!("Executing command...");
//                     // }
//                     KeyCode::Char('d') => {
//                         // Example: Detect 'exit' command to quit the session
//                         println!("Exiting interactive session.");
//                         break;
//                     }
//                     _ => {
//                         // Handle other key events
//                         println!("You pressed: {:?}", code);
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
//
//     println!("DONE");
//     return Ok(());
//
// }
