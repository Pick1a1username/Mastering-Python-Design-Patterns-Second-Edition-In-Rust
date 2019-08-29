// Since there is no crate like Pickle in Rust,
// I created something called 'pseudo_pickle', which behaves same as
// Pickle in the Python code.
use memento::pseudo_pickle;
use memento::pseudo_pickle::Pickle;

#[derive(Clone)]
struct Quote {
    text: String,
    author: String,
}

impl Quote {
    fn new(text: String, author: String) -> Quote {
        Quote { text: text, author: author }
    }

    fn save_state(&self) -> Pickle<Quote>
    {
        let current_state = pseudo_pickle::dumps(self.clone());

        return current_state;
    }

    fn restore_state(&mut self, memento: Pickle<Quote>) {
        let previous_state = pseudo_pickle::loads(memento);

        self.text = previous_state.text;
        self.author = previous_state.author;
    }

    fn get_info(&self) -> String {
        return String::from(format!("{} - By {}", self.text, self.author));
    }
}


fn main() {
    println!("Quote 1");
    let mut q1 = Quote::new("A room without books is like a body without a soul.".to_string(), 
               "Unknown author".to_string());
    println!("\nOriginal version:\n{}", q1.get_info());
    let q1_mem = q1.save_state();

    // Now, we found the author's name
    q1.author = "Marcus Tullius Cicero".to_string();
    println!("\nWe found the author, and did an updated:\n{}", q1.get_info());

    // Restoring previous state (Undo)
    q1.restore_state(q1_mem);
    println!("\nWe had to restore the previous version:\n{}", q1.get_info());

    println!();
    println!("Quote 2");
    let mut q2 = Quote::new("To be you in a world that is constantly trying to make you be something else is the greatest accomplishment.".to_string(),
               "Ralph Waldo Emerson".to_string());
    println!("\nOriginal version:\n{}", q2.get_info());
    let q2_mem1 = q2.save_state();

    // changes to the text
    q2.text = "To be yourself in a world that is constantly trying to make you something else is the greatest accomplishment.".to_string();
    println!("\nWe fixed the text:\n{}", q2.get_info());
    let q2_mem2 = q2.save_state();

    q2.text = "To be yourself when the world is constantly trying to make you something else is the greatest accomplishment.".to_string();
    println!("\nWe fixed the text again:\n{}", q2.get_info());

    // Restoring previous state (Undo)
    q2.restore_state(q2_mem2);
    println!("\nWe had to restore the 2nd version, the correct one:\n{}", q2.get_info());
}

