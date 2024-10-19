pub mod requests;
use requests::request_handler::{App, Method};

fn main() {
    let mut app = App::new();

    fn test(content: &str) -> String {
        String::from(content)
    }

    app.create("/otra-ruta/", Method::POST, test);
    app.listen(3000);
}
