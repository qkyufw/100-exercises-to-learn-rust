// TODO: Implement the `in_progress` method. It must return an iterator over the tickets in
//  `TicketStore` with status set to `Status::InProgress`.
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
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
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    /// 返回一个迭代器，遍历所有状态为 `InProgress` 的 ticket
    ///
    /// # 使用 `impl Trait` 的原因
    ///
    /// 如果我们试图写出具体的返回类型，会是这样的：
    ///   `std::iter::Filter<std::slice::Iter<'_, Ticket>, [闭包类型]>`
    ///
    /// 但闭包是匿名类型，无法在代码中写出它的名字。
    /// 因此我们使用 `impl Iterator<Item = &Ticket>` 来表示：
    /// "返回某个实现了 `Iterator<Item = &Ticket>` 的类型"
    ///
    /// 这是 **返回位置的 impl Trait**，它的特点是：
    /// - 返回类型由函数实现固定，不是泛型
    /// - 允许返回那些"存在但无法命名"的类型
    /// - 编译器知道确切的类型，只是对外不透明
    ///
    /// 等价于下面这个无法编译的写法：
    /// ```ignore
    /// pub fn in_progress(&self) -> Filter<Iter<'_, Ticket>, Closure> {
    ///     self.tickets.iter().filter(|t| t.status == Status::InProgress)
    /// }
    /// ```
    pub fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
        // `self.tickets.iter()` 返回 `Iter<'_, Ticket>`
        // `.filter(...)` 返回 `Filter<Iter<'_, Ticket>, Closure>`
        // 闭包 `|t| t.status == Status::InProgress` 捕获 `InProgress` 状态进行筛选
        self.tickets.iter().filter(|t| t.status == Status::InProgress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn in_progress() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(todo);

        let in_progress = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(in_progress.clone());

        let in_progress_tickets: Vec<&Ticket> = store.in_progress().collect();
        assert_eq!(in_progress_tickets.len(), 1);
        assert_eq!(in_progress_tickets[0], &in_progress);
    }
}
