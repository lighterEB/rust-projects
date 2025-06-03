use std::fmt;
use std::io::{self};

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

        for (_index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✅" } else { "⏳" };
            println!(
                "{} | {} {} {} | {}",
                task.id,
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
                "{} | {} {} {} | {}",
                task.id,
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
    fn delete_task(&mut self, index: usize) -> Result<(), TodoError> {
        if index >= self.tasks.len() || index == 0 {
            return Err(TodoError::InvalidIndex);
        }
        // self.tasks.remove(index);
        let task = self.tasks.remove(index);
        println!("🗑️ 已删除任务: '{}'", task.description);
        Ok(())
    }

    // 批量删除已完成任务
    fn delete_complete_task(&mut self) -> Result<(), TodoError> {
        if self.tasks.is_empty() {
            return Err(TodoError::NoTask);
        }

        let task_count = self.tasks.len();
        self.tasks.retain(|task| !task.completed);
        let delete_task = task_count - self.tasks.len();
        if delete_task == 0 {
            return Err(TodoError::NoCompletedTask);
        }
        println!("🗑️ 已删除 {} 个已完成的任务", delete_task);
        // 收集已完成任务的索引，从小到大
        // let mut indices: Vec<usize> = self
        //     .tasks
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, task)| task.completed)
        //     .map(|(index, _)| index)
        //     .collect();
        // // 按降序排序确保从后向前删除
        // indices.sort_by(|a, b| b.cmp(a));
        // // 记录删除任务的数量
        // let count = indices.len();
        // if count == 0 {
        //     return Err("没有已完成的任务可删除！");
        // }
        // for index in indices {
        //     println!("删除已完成的任务编号：{}", index);
        //     match self.delete_task(index) {
        //         Ok(_) => continue,
        //         Err(e) => return Err(e),
        //     }
        // }
        Ok(())
    }

    // 搜索任务
    fn search_tasks(&self, keyword: &str) -> Result<(), TodoError> {
        let matching_tasks: Vec<_> = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| {
                task.description
                    .to_lowercase()
                    .contains(&keyword.to_lowercase())
            })
            .collect();
        if matching_tasks.len() == 0 {
            println!("🔍 没有找到包含 '{}' 的任务", keyword);
            return Ok(());
        }
        println!("\n🔍 搜索结果 (关键词: '{}'):", keyword);
        println!("{:-<60}", "");
        for (_index,task) in matching_tasks {
            let status = if task.completed { "✅" } else { "⏳" };
            println!(
                "{} | {} {} {} | {}",
                task.id,
                status,
                task.priority.to_emoji(),
                task.priority.to_string(),
                task.description
            );
        }
        println!("{:-<60}", "");
        Ok(())
    }

    // 统计信息
    fn show_stats(&self) {
        // 任务总数
        let total = self.tasks.len();
        // 已完成的任务
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        // 待办
        let pending = total - completed;

        // 任务优先级
        let high_priority = self.tasks.iter().filter(|t| matches!(t.priority, Priority::High) && !t.completed).count();
        let medium_priority = self.tasks.iter().filter(|t| matches!(t.priority, Priority::Medium) && !t.completed).count();
        let low_priority = self.tasks.iter().filter(|t| matches!(t.priority, Priority::Low) && !t.completed).count();

        println!("\n📊 任务统计:");
        println!("总任务数: {}", total);
        println!("已完成: {} ✅", completed);
        println!("待完成: {} ⏳", pending);
        println!("高优先级待办: {} 🔴", high_priority);
        println!("中优先级待办: {} 🟡", medium_priority);
        println!("低优先级待办: {} 🟢", low_priority);
    }
}

// 获取用户输入
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string()
}

// 显示菜单
fn show_menu() {
    println!("\n🎯 Rust任务管理器");
    println!("=================");
    println!("1. 添加任务");
    println!("2. 查看所有任务");
    println!("3. 按优先级查看任务");
    println!("4. 完成任务");
    println!("5. 删除任务");
    println!("6. 删除所有已完成任务");
    println!("7. 搜索任务");
    println!("8. 查看统计");
    println!("0. 退出");
    println!("=================");
}

fn main() {
    // let mut todo = TodoList::new();
    // todo.add_task(String::from("打飞机"), "High");
    // todo.add_task(String::from("做作业"), "Medium");
    // todo.show_stats();
    let mut todo = TodoList::new();

    // 添加一些示例数据
    let _ = todo.add_task("研究人类的诞生".to_string(), "high");
    let _ = todo.add_task("吃一份番茄蛋饭".to_string(), "medium");
    let _ = todo.add_task("对着天空说520".to_string(), "low");

    println!("🚀 欢迎使用Rust任务管理器！");

    loop {
        show_menu();
        let choice = get_input("请选择操作(0-8):");

        match choice.as_str() {
            "1" => {
                let description = get_input("请输入任务描述：");
                if description.is_empty() {
                    println!("🙅任务描述不能为空！");
                    continue;
                }

                let priority = get_input("请输入优先级（high/medium/low 或 h/m/l 或 1/2/3）:");
                match todo.add_task(description, &priority) {
                    Ok(_) => {},
                    Err(e) => println!("🙅‍♂️{}", e),
                }
            },
            "2" => {
                if let Err(e) = todo.list_tasks() {
                    println!("🙅‍♂️{}", e);
                }
            },
            "3" => {
                if let Err(e) = todo.list_task_by_priority() {
                    println!("🙅‍♂️{}", e);
                }
            },
            "4" => {
                if let Ok(_) = todo.list_tasks() {
                    let input = get_input("请输入要完成的任务编号：");
                    match input.parse::<usize>() {
                        Ok(index) => {
                            if let Err(e) = todo.complete_task(index) {
                                println!("🙅‍♂️{}", e);
                            }
                        }
                        Err(_) => println!("🙅‍♂️请输入有效数字！"),
                    }
                }
            },
            "5" => {
                if let Ok(_) = todo.list_tasks() {
                    let input = get_input("请输入要删除的任务编号：");
                    match input.parse::<usize>() {
                        Ok(index) => {
                            if let Err(e) = todo.delete_task(index) {
                                println!("🙅‍♂️{}", e);
                            }
                        }
                        Err(_) => println!("🙅‍♂️请输入有效数字！"),
                    }
                }
            },
            "6" => {
                if let Err(e) = todo.delete_complete_task() {
                    println!("🙅‍♂️{}", e);
                }
            },
            "7" => {
                let keyword = get_input("请输入搜索关键字：");
                if !keyword.is_empty() {
                    let _ = todo.search_tasks(&keyword);
                }
            },
            "8" => {
                todo.show_stats();
            },
            "0" => {
                println!("👋 再见！感谢使用任务管理器！");
                break;
            },
            _ => {
                println!("无效选择，请重新输入！");
            }

        }

        // 按任意键继续
        let _ = get_input("\n按回车键继续...");
    }
}
