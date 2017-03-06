extern crate pencil;

use std::io::Read;
use pencil::{Pencil, Request, Response, PencilResult};
use pencil::PathBound;

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn main() {
    let mut app = Pencil::new("resources");

    let mut file = app.open_resource("list.txt");
    let mut content = String::from("");
    file.read_to_string(&mut content).unwrap();

    print!("{}", content);
    app.get("/", "hello", hello);
    app.run("127.0.0.1:5000");
}
