use db::db::{load, print_lib};

pub mod db;
pub mod player;

fn main() {
    load(true);
    print_lib();
}
