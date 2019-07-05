use std::fmt;
use std::rc::Rc;

// Define events can be handled.
trait Down {}

trait Paint {}

// trait Unhandled;

trait Close {}

// https://doc.rust-lang.org/beta/rust-by-example/error/multiple_error_types/define_error_type.html
struct UnhandledEvent;

// Define events
enum Event {
    Down,
    Paint,
    // Unhandled,
    Close,
}

trait Widget {
    fn handle(&self, event: Event) -> Result<&str, UnhandledEvent>;
}

// Define MainWindow widget.
struct MainWindow {
    parent: Option<Rc<Widget>>,
}

impl Widget for MainWindow {
    fn handle(&self, event: Event) -> Result<&str, UnhandledEvent> {
        match event {
            Event::Close => {
                Ok("MainWindow: Close")
            },
            _ => {
                Err(UnhandledEvent)
            },
        }
    }
}

impl MainWindow {
    fn new(parent: Option<Rc<Widget>>) -> Self {
        MainWindow { parent: parent }
    }
}


fn main() {
    let main_window = MainWindow::new(None);
    println!("{}", main_window.handle(Event::Down).unwrap_or("Unhandled"));
    println!("{}", main_window.handle(Event::Close).unwrap_or("Unhandled"));
}
