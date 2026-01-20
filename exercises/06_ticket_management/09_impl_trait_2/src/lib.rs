// TODO: Rework the signature of `TicketStore::add_ticket` to use a generic type parameter rather
//  than `impl Trait` syntax.

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

    // 使用 `Into<Ticket>` 作为类型参数，使方法可以接受任何能够转换为 `Ticket` 的类型
    // 这可以调用时更简洁，不需要显式调用 `.into()`
    //
    // # 为什么用泛型而不是 `impl Trait`？
    //
    // 参数位置的 `impl Trait` 完全等价于泛型：
    //   `ticket: impl Into<Ticket>`  等价于  `<T: Into<Ticket>>(ticket: T)`
    //
    // 但推荐使用显式泛型的原因是：
    /// 1. 允许使用 turbofish 语法显式指定类型参数
    /// 2. 当类型推断有歧义时可以消除歧义
    /// 3. API 更清晰明确
    //
    // 例如测试代码中使用了：
    //   `store.add_ticket::<TicketDraft>(draft)`
    // 这种写法只有泛型参数才支持，`impl Trait` 不支持。
    pub fn add_ticket<T>(&mut self, ticket: T)
    where
        T: Into<Ticket>,
    {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // This won't compile if `add_ticket` uses `impl Trait` syntax in argument position.
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
