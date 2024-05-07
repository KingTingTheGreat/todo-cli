mod colors;
mod date;
mod entry;
mod status;
mod todo_list;

use crate::todo_list::SortType::*;
use clap::Parser;
use date::Date;
use entry::*;
use status::Status;
use todo_list::TodoList;

const ENTRIES_FILENAME: &str = "~/todo-entries.json";

fn init_saved() -> TodoList {
    let todo_list = TodoList::from_file(ENTRIES_FILENAME);
    return todo_list;
}

fn persist(todo_list: &TodoList) {
    todo_list.to_file(ENTRIES_FILENAME);
}

// #[derive(Parser, Default, Debug)]
// struct Arguments {
//     #[clap(short, long, default_value_t = false)]
//     /// if areas.csv to download
//     areas: bool,
//     #[clap(short, long, default_value_t = false)]
//     /// if markers.csv to download
//     markers: bool,
//     #[clap(short, long, default_value_t = false)]
//     /// if tracks.csv to download
//     tracks: bool,
//     #[clap(short, long)]
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
//     /// path to file of GPS tracks to download
//     gpx_list_file: Option<String>,
// }

#[derive(Parser, Debug)]
enum SubCommand {
    Add(Add),
    Update(Update),
    Remove(Remove),
    Sort(Sort),
    Reverse,
}

#[derive(Parser, Debug)]
struct Add {
    name: String,
    #[clap(short, long, default_value_t = String::new())]
    description: String,
    #[clap(short, long, default_value_t = String::new())]
    category: String,
    #[clap(short='t', long, default_value_t = Date::now().add_days(7).to_string())]
    due_date: String,
    #[clap(short, long, default_value_t = Status::NotStarted.to_string())]
    status: String,
}

#[derive(Parser, Debug)]
struct Update {
    id: i32,
    name: Option<String>,
    description: Option<String>,
    due_date: Option<String>,
    category: Option<String>,
    status: Option<String>,
}

#[derive(Parser, Debug)]
struct Remove {
    id: usize,
}

#[derive(Parser, Debug)]
struct Sort {
    attr: String,
}

// #[derive(Parser)]
// struct Find {
//     pattern: String,
// }

#[derive(Parser, Default, Debug)]
struct Args {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

fn main() {
    let mut todo_list = init_saved();

    let args = Args::parse();
    match args.subcmd {
        Some(SubCommand::Add(add)) => {
            let due_date = Date::from_string(&add.due_date);
            let status = Status::from_string(&add.status);
            let entry = Entry::new(add.name, add.description, due_date, add.category, status);
            todo_list.add_entry(entry);
        }
        // Some(SubCommand::Update(update)) => {
        //     let mut entry = todo_list.get_entry(update.id);
        //     if let Some(name) = update.name {
        //         entry.set_name(name);
        //     }
        //     if let Some(description) = update.description {
        //         entry.set_description(description);
        //     }
        //     if let Some(due_date) = update.due_date {
        //         entry.set_due_date(Date::from_string(&due_date));
        //     }
        //     if let Some(category) = update.category {
        //         entry.set_category(category);
        //     }
        //     if let Some(status) = update.status {
        //         entry.set_status(Status::from_string(&status));
        //     }
        // }
        Some(SubCommand::Remove(remove)) => {
            todo_list.remove_entry(remove.id - 1);
        }
        Some(SubCommand::Sort(sort)) => {
            let lower_attr = sort.attr.to_lowercase();
            let trim_attr = lower_attr.trim();
            if ["name", "n"].contains(&trim_attr) {
                todo_list.sort(Name);
            } else if ["date added", "date-added", "da"].contains(&trim_attr) {
                todo_list.sort(DateAdded);
            } else if ["due date", "due-date", "dd"].contains(&trim_attr) {
                todo_list.sort(DueDate);
            } else if ["category", "c"].contains(&trim_attr) {
                todo_list.sort(Category);
            } else if ["status", "s"].contains(&trim_attr) {
                todo_list.sort(Status);
            } else {
                println!("Invalid sort type")
            }
        }
        Some(SubCommand::Reverse) => {
            todo_list.reverse();
        }
        _ => {}
    }

    // let args = Cli::parse();
    // println!("Pattern: {}", args.pattern);
    // println!("Path: {:?}", args.path);

    // let args = Find::parse();
    todo_list.print_entries(&[NAME, DESCRIPTION, CATEGORY, DUE_DATE, STATUS]);

    persist(&todo_list);

    let x = Date::now();
    println!("{}", x.to_string());

    let y = Status::NotStarted;
    println!("{}", y.to_string());
}
