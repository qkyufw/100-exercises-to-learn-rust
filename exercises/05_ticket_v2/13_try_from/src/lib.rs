// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

// 场景A：类型转换可能会丢失数据，比如将数字转化为错误，一个超出 i32 的整数就会丢失数据
// 场景B：解析/反序列化可能失败
// 场景C：验证复杂业务逻辑
// 场景D：复杂结构的转换
// TryFrom：明确目标的类型的时候使用
// TryInto：不知道目标的类型，进行推导
impl TryFrom<String> for Status {
    type Error = String; // 必须指定关联类型 Error，type Output = i32，这种也是关联类型
                         // 关联类型：每个实现固定一个错误类型，更清晰，调用者知道会得到什么错误，也避免复杂泛型参数
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() { // 大小写不敏感
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {}", value)),  // 无效输入
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
