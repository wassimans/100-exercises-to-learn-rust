// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title_empty(&title) {
            panic!("Title cannot be empty");
        }
        if title_long(&title) {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description_empty(&description) {
            panic!("Description cannot be empty");
        }
        if description_long(&description) {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status_valid(&status) {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_title(&mut self, new_title: String) {
        if title_empty(&new_title) {
            panic!("Title cannot be empty");
        }
        if title_long(&new_title) {
            panic!("Title cannot be longer than 50 bytes");
        }
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        if description_empty(&new_description) {
            panic!("Description cannot be empty");
        }
        if description_long(&new_description) {
            panic!("Description cannot be longer than 500 bytes");
        }
        self.description = new_description;
    }

    pub fn set_status(&mut self, new_status: String) {
        if status_valid(&new_status) {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        self.status = new_status
    }
}

fn title_empty(title: &str) -> bool {
    title.is_empty()
}

fn title_long(title: &str) -> bool {
    title.len() > 50
}

fn description_empty(description: &str) -> bool {
    description.is_empty()
}

fn description_long(description: &str) -> bool {
    description.len() > 500
}

fn status_valid(status: &str) -> bool {
    status != "To-Do" && status != "In Progress" && status != "Done"
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
