use std::error::Error;
use std::rc::Rc;

use console::ConsoleMenu;
use lab::repository::{FileRepository, Repository};

fn main() -> Result<(), Box<dyn Error>> {
    // create repo (IoC -> pass trait object)
    let repo: Rc<dyn Repository> = Rc::new(FileRepository::new());
    let menu = ConsoleMenu::new(repo, "data");
    if let Err(e) = menu.run() {
        eprintln!("Error during run: {}", e);
    }
    Ok(())
}
