// TODO: Replace `todo!()`s with the correct implementation.
//  Implement additional traits on `TicketId` if needed.

use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u64,
}

// TicketId 作为 HashMap 的 key，必须实现以下 trait：
// - PartialEq: 可比较相等性
// - Eq: 保证自反性（a == a 永远为 true）
// - Hash: 可计算哈希值，HashMap 用它来快速定位数据
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
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

        // 3. 插入到 HashMap 中
        // HashMap::insert 返回 Option<V>：如果 key 已存在，返回旧值；否则返回 None
        self.tickets.insert(id, ticket);

        // 4. 返回 ID
        id
    }

    // 获取 ticket 的不可变引用
    // HashMap::get 返回 Option<&V>，O(1) 时间复杂度
    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)  // 注意：需要传 &id，因为 get 的参数是 &Q
    }

    // 获取 ticket 的可变引用
    // HashMap::get_mut 返回 Option<&mut V>，O(1) 时间复杂度
    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id = store.add_ticket(draft.clone());
        let ticket = &store[id];
        assert_eq!(draft.title, ticket.title);
        assert_eq!(draft.description, ticket.description);
        assert_eq!(ticket.status, Status::ToDo);

        let ticket = &mut store[id];
        ticket.status = Status::InProgress;

        let ticket = &store[id];
        assert_eq!(ticket.status, Status::InProgress);
    }
}
