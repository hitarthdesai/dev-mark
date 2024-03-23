use tokio_postgres::{Error};
use crate::argument::Arguments;
use crate::db::Database;
use crate::util::mark::Mark;
use termion::color::{Blue, Fg, Red, Reset};

pub async fn list_marks(db: &Database, args: &Arguments) -> Result<(), Error> {
    let _marks = db.read_marks_by_date(&args.date).await?;
    let marks: Vec<Mark> = _marks.iter().map(Mark::new_from_row).collect();

    if marks.len() == 0 {
        println!("You have no marks for {}", args.date.format("%B %e, %Y").to_string());
        return Ok(());
    }


    println!("Your marks for {}{}{}:", Fg(Red), args.date.format("%B %e, %Y").to_string(), Fg(Reset));
    marks.iter().for_each(|mark| {
        println!("{}{}{}: {}", Fg(Blue), mark.created_at.format("%H:%M").to_string(), Fg(Reset), mark.note);
    });

    Ok(())
}
