use std::fmt;
use std::fmt::write;
use std::io::{self, Error};
use std::mem::take;

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
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    priority: Priority,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String, priority: Priority) -> Self {
        Task {
            id,
            description,
            priority,
            completed: false,
        }
    }
}

// 任务管理器结构体
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    // 创建空的代办事项列表
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // 添加任务
    fn add_task(&mut self, description: String, priority_str: &str) -> Result<(), TodoError> {
        // let priority = match priority.to_lowercase().as_str() {
        //     "high" => Priority::High,
        //     "medium" => Priority::Medium,
        //     "low" => Priority::Low,
        //     _ => {
        //         println!("无效的优先级，将使用默认值 'Low'");
        //         Priority::Low
        //     }
        // };
        let priority = Priority::from_str(priority_str)?;
        let task = Task::new(self.next_id, description, priority);
        self.tasks.push(task);
        self.next_id += 1;
        // let task = Task {
        //     description,
        //     priority,
        //     completed: false,
        // };

        // self.tasks.push(task);
        println!("任务已添加！👌");
        Ok(())
    }

    // 列出所有任务
    fn list_tasks(&self) -> Result<(), TodoError> {
        if self.tasks.is_empty() {
            return Err(TodoError::NoTask);
        }
        println!("\n📋当前任务列表：");
        println!("{:-<60}", "");

        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✅" } else { "⏳" };
            println!(
                "{} | {} {} {} | {}",
                index + 1,
                status,
                task.priority.to_emoji(),
                task.priority.to_string(),
                task.description
            );
        }
        // for (index, task) in self.tasks.iter().enumerate() {
        //     let status = if task.completed { "✅" } else { "🈚️" };
        //     println!(
        //         "编号：{} | 描述：{} | 优先级：{:?} | 状态：{}",
        //         index, task.description, task.priority, status
        //     );
        // }

        println!("{:-<60}", "");
        Ok(())
    }

    // 按照优先级列出任务
    fn list_task_by_priority(&self) -> Result<(), TodoError> {
        if self.tasks.is_empty() {
            return Err(TodoError::NoTask);
        }

        let mut sorted_tasks = self.tasks.clone();
        sorted_tasks.sort_by(|a, b| {
            use Priority::*;
            let order = |p: &Priority| match p {
                High => 0,
                Medium => 1,
                Low => 2,
            };
            order(&a.priority).cmp(&order(&b.priority))
        });

        println!("\n📋 按优先级排序的任务列表：");
        println!("{:-<60}", "");

        for task in sorted_tasks {
            let status = if task.completed { "✅" } else { "⏳" };

            println!(
                "{} {} {} | {}",
                status,
                task.priority.to_emoji(),
                task.priority.to_string(),
                task.description
            );
        }
        println!("{:-<60}", "");
        Ok(())
    }

    // 标记任务为已完成
    // fn complete_task(&mut self, index: usize) -> Result<(), &'static str> {
    //     if index >= self.tasks.len() {
    //         return Err("任务编号无效！");
    //     }
    //     self.tasks[index].completed = true;
    //     Ok(())
    // }
    fn complete_task(&mut self, index: usize) -> Result<(), TodoError> {
        if index == 0 || index > self.tasks.len() {
            return Err(TodoError::InvalidIndex);
        }

        let task_index = index - 1;
        if self.tasks[task_index].completed {
            println!("⚠️ 任务已经完成！");
            return Ok(());
        }

        self.tasks[task_index].completed = true;
        println!(
            "✅ 任务 '{}' 已标记为完成！",
            self.tasks[task_index].description
        );

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
