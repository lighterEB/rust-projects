use std::fmt;
use std::fmt::write;
use std::io;

// 自定义错误类型
#[derive(Debug)]
enum TodoError {
    InvalidIndex,
    NoTask,
    NoCompletedTask,
    InvalidPriority(String),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::InvalidIndex => write!(f, "任务编号无效！"),
            TodoError::NoTask => write!(f, "没有任务"),
            TodoError::NoCompletedTask => write!(f, "没有已完成的任务可以删除！"),
            TodoError::InvalidPriority(p) => write!(f, "无效的优先级{}，请使用 high/medium/low", p),
        }
    }
}

// 定义优先级枚举
#[derive(Debug, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    fn from_str(s: &str) -> Result<Priority, TodoError> {
        match s.to_lowercase().as_str() {
            "high" | "高" | "h" | "1" => Ok(Priority::High),
            "medium" | "中" | "m" | "2" => Ok(Priority::Medium),
            "low" | "低" | "l" | "3" => Ok(Priority::Low),
            _ => Err(TodoError::InvalidPriority(s.to_string())),
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Priority::High => "高",
            Priority::Medium => "中",
            Priority::Low => "低",
        }
    }

    fn to_emoji(&self) -> &str {
        match self {
            Priority::High => "🔴",
            Priority::Medium => "🟡",
            Priority::Low => "🟢",
        }
    }
}

// 定义任务结构体
#[derive(Debug)]
struct Task {
    description: String,
    priority: Priority,
    completed: bool,
}

// 任务管理器结构体
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    // 创建空的代办事项列表
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    // 添加任务
    fn add_task(&mut self, description: String, priority: &str) {
        let priority = match priority.to_lowercase().as_str() {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => {
                println!("无效的优先级，将使用默认值 'Low'");
                Priority::Low
            }
        };
        let task = Task {
            description,
            priority,
            completed: false,
        };

        self.tasks.push(task);
        println!("任务已添加！");
    }

    // 列出所有任务
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("没有任务！");
            return;
        }
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✅" } else { "🈚️" };
            println!(
                "编号：{} | 描述：{} | 优先级：{:?} | 状态：{}",
                index, task.description, task.priority, status
            );
        }
    }

    // 标记任务为已完成
    fn complete_task(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.tasks.len() {
            return Err("任务编号无效！");
        }
        self.tasks[index].completed = true;
        Ok(())
    }

    // 删除任务
    fn delete_task(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.tasks.len() {
            return Err("任务编号无效！");
        }
        self.tasks.remove(index);
        Ok(())
    }

    // 批量删除已完成任务
    fn delete_complete_task(&mut self) -> Result<(), &'static str> {
        if self.tasks.is_empty() {
            return Err("没有任务可删除");
        }
        // 收集已完成任务的索引，从小到大
        let mut indices: Vec<usize> = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.completed)
            .map(|(index, _)| index)
            .collect();
        // 按降序排序确保从后向前删除
        indices.sort_by(|a, b| b.cmp(a));
        // 记录删除任务的数量
        let count = indices.len();
        if count == 0 {
            return Err("没有已完成的任务可删除！");
        }
        for index in indices {
            println!("删除已完成的任务编号：{}", index);
            match self.delete_task(index) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

fn main() {
    let mut todo = TodoList::new();
    todo.add_task(String::from("打飞机"), "High");
    todo.add_task(String::from("做作业"), "Medium");
    // println!("任务列表: {:?}", todo.tasks);
    todo.list_tasks();
    match todo.complete_task(0) {
        Ok(_) => println!("任务已标记为完成！"),
        Err(e) => println!("错误：{}", e),
    }

    match todo.complete_task(1) {
        Ok(_) => println!("任务已标记为完成！"),
        Err(e) => println!("错误：{}", e),
    }
    todo.list_tasks();
    match todo.delete_complete_task() {
        Ok(_) => println!("所有完成任务已删除"),
        Err(e) => println!("错误：{}", e),
    }
    println!("目前未完成任务清单：");
    todo.list_tasks();
}
