pub mod requests;
use requests::request_handler::{App, Method};

fn main() {
    let mut app = App::new();

    fn test() -> String {
        String::from("I'm a beast")
    }

    app.create("/otra-ruta/", Method::POST, test);
    app.listen(3000);
}
