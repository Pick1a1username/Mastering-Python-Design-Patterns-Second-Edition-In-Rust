use std::rc::Rc;
use std::fmt;


// https://doc.rust-lang.org/beta/rust-by-example/error/multiple_error_types/define_error_type.html
#[derive(Debug)]
struct UnhandledEvent;

// Define events
#[derive(Debug, Copy, Clone)]
enum Event {
    Down,
    Paint,
    Unhandled,
    Close,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


trait Widget {
    fn handle(&self, event: Event) -> Result<String, UnhandledEvent>;
    fn handle_default(&self, event: Event) -> Result<String, UnhandledEvent> {
        Err(UnhandledEvent)
    }
}

struct MainWindow {
    parent: Option<Rc<Box<dyn Widget>>>,
}

impl Widget for MainWindow {
    fn handle(&self, event: Event) -> Result<String, UnhandledEvent> {
        match event {
            Event::Close => {
                Ok(format!("MainWindow: {}", event))
            },
            _ => {
                self.handle_default(event)
            },
        }
    }
    
    // Override trait's function
    fn handle_default(&self, event: Event) -> Result<String, UnhandledEvent> {
        // Todo: Print event's name
        Ok(format!("MainWindow Default: {}", event))
    }
}

impl MainWindow {
    fn new(parent: Option<Rc<Box<dyn Widget>>>) -> Self {
        MainWindow { parent: parent }
    }
}

struct SendDialog<T> {
    parent: Option<Rc<Box<T>>>,
}

impl<T: Widget> Widget for SendDialog<T> {
    fn handle(&self, event: Event) -> Result<String, UnhandledEvent> {
        match event {
            Event::Paint => {
                Ok(format!("SendDialog: {}", event))
            },
            _ => {
                match &self.parent {
                    Some(parent) => parent.handle(event),
                    None => Err(UnhandledEvent),
                }
            },
        }    
    }
}

impl<T> SendDialog<T> {
    fn new(parent: Option<Rc<Box<T>>>) -> Self {
        SendDialog { parent: parent }
    }
}

struct MsgText<T> {
    parent: Option<Rc<Box<T>>>,
}

impl<T: Widget> Widget for MsgText<T> {
    fn handle(&self, event: Event) -> Result<String, UnhandledEvent> {
        match event {
            Event::Down => {
                Ok(format!("MsgText: {}", event))
            },
            _ => {
                match &self.parent {
                    Some(parent) => parent.handle(event),
                    None => Err(UnhandledEvent),
                }
            },
        }    
    }
}

impl<T> MsgText<T> {
    fn new(parent: Option<Rc<Box<T>>>) -> Self {
        MsgText { parent: parent }
    }
}

fn main() {
    let mw = Rc::new(Box::new(MainWindow::new(None)));
    let sd = Rc::new(Box::new(SendDialog::new(Some(Rc::clone(&mw)))));
    let msg = Rc::new(Box::new(MsgText::new(Some(Rc::clone(&sd)))));
 
    let events = vec![Event::Down, Event::Paint, Event::Unhandled, Event::Close];

    for e in events {
        println!("Sending event -{}- to MainWindow", e);
        println!("{}", mw.handle(e).unwrap());
        println!("Sending event -{}- to SendDialog", e);
        println!("{}", sd.handle(e).unwrap());
        println!("Sending event -{}- to MsgText", e);
        println!("{}", msg.handle(e).unwrap());
    }
}
