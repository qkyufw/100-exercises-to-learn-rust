// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Ticket { title: self.title.clone(), description: self.description.clone(), status: self.status.clone() }
    }
}

// #[derive(Clone)]  // ← 最简单：自动实现
// pub struct Ticket {
//     pub title: String,
//     pub description: String,
//     pub status: String,
// }

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let ticket_clone = ticket.clone();
    (ticket_clone, ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
