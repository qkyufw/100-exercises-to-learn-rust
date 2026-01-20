// TODO: Implement `Index<&TicketId>` and `Index<TicketId>` for `TicketStore`.

use std::ops::Index;

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    counter: u64,
}

// 为 TicketStore 实现 Index trait，支持使用 &TicketId 进行索引
// 例如: &store[&id]
impl Index<&TicketId> for TicketStore {
    // 关联类型：索引后返回的类型是 Ticket 的引用
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        // 复用 get 方法查找 ticket
        // *index 解引用，将 &TicketId 转换为 TicketId
        // expect 在 Option 为 None 时会 panic（Index trait 的约定）
        self.get(*index).expect("Ticket not found")
    }
}

// 为 TicketStore 实现 Index trait，支持使用 TicketId 进行索引
// 例如: &store[id]
impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        // 直接使用 get 方法查找
        // expect 在找不到时 panic，这是 Index trait 的标准行为
        self.get(index).expect("Ticket not found")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.push(ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|&t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = &store[id1];
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = &store[&id2];

        assert_ne!(id1, id2);
    }
}
