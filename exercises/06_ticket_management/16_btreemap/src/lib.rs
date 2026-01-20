// TODO: Replace `todo!()`s with the correct implementation.
//  Implement `IntoIterator` for `&TicketStore`. The iterator should yield immutable
//  references to the tickets, ordered by their `TicketId`.
//  Implement additional traits on `TicketId` if needed.

use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use std::iter::IntoIterator;
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

// BTreeMap 的 key 必须实现 Ord trait（而 HashMap 需要 Hash + Eq）
// Ord 允许 BTreeMap 按照键的顺序存储和遍历
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            tickets: BTreeMap::new(),  // 创建空的 BTreeMap
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        // 1. 生成新的 ID
        let id = TicketId(self.counter);
        self.counter += 1;

        // 2. 创建 Ticket
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };

        // 3. 插入到 BTreeMap
        // BTreeMap 会自动按照 TicketId 的顺序存储
        self.tickets.insert(id, ticket);

        // 4. 返回 ID
        id
    }

    // 获取 ticket 的不可变引用，O(log n) 时间复杂度
    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    // 获取 ticket 的可变引用，O(log n) 时间复杂度
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

// 实现 IntoIterator for &TicketStore
// 这样就可以用 &store 进行迭代，获取所有 tickets
impl<'a> IntoIterator for &'a TicketStore {
    // Item 是迭代器产生的元素类型：&Ticket
    type Item = &'a Ticket;
    // IntoIter 是迭代器类型
    type IntoIter = TicketStoreIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // BTreeMap 的 values() 方法已经按 key 的顺序返回值
        TicketStoreIterator {
            inner: self.tickets.values(),
        }
    }
}

// 自定义迭代器类型，包装 BTreeMap 的值迭代器
pub struct TicketStoreIterator<'a> {
    // 内部使用 BTreeMap 的 Values 迭代器
    inner: std::collections::btree_map::Values<'a, TicketId, Ticket>,
}

// 为迭代器实现 Iterator trait
impl<'a> Iterator for TicketStoreIterator<'a> {
    type Item = &'a Ticket;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()  // 委托给内部的 BTreeMap 迭代器
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketId, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let n_tickets = 5;

        for i in 0..n_tickets {
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

        let ids: Vec<TicketId> = (&store).into_iter().map(|t| t.id).collect();
        let sorted_ids = {
            let mut v = ids.clone();
            v.sort();
            v
        };
        assert_eq!(ids, sorted_ids);
    }
}
