use db::setup_db::{load, print_lib};

pub mod db;

fn main() {
    load(true);
    print_lib();
}
