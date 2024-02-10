# Dev Mark

A command-line note-taking tool to mark important things throughout your day.

## Commands

1. `mark` - Add a new mark
    - needs a date and time to add a mark
    - provide a date using `--today`, `--today+{int}`, `--today-{int}`
    - provide a time using `--time=HH:MM or HH:MM:SS or HH-MM or HH-MM-SS`
    - if no date or time is provided, a date must be entered using the date picker
        or time using the text field respectively.
2. `marks` - List all marks
    - needs a date to list marks for that date (same constraints as `mark` command)
3. `unmark` - Remove a mark
    - needs a date to list marks from which selection to delete needs to be made
    - one can multi-select marks to delete (same constraints as `mark` command)

## Instructions to run locally
1. Clone the repo and install dependencies using `cargo build`
2. Create a psql database and record its host, port, and name
3. Create a `config.json` file at the root of the project with the following structure:
    ```json
    {
        "connect_string": "host=localhost port=15432 dbname=postgres"
    }
    ```
4. Run the binary using `cargo run <command> <args>`