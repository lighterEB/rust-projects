use slint::slint;
use std::cell::RefCell;
use std::rc::Rc;

slint! {
    import { Button, VerticalBox, HorizontalBox } from "std-widgets.slint";

    export component Calculator inherits Window {
        width: 300px;
        height: 400px;
        background: #001000;
        title: "计算器";

        callback append_digit(string);
        callback append_dot();
        callback get_result();
        callback operation(string);
        callback clearly();
        // 定义属性存储显示文本
        in-out property <string> display_text: "0";

        VerticalBox {
            spacing: 10px;
            padding: 10px;

            // 显示区域
            Rectangle {
                background: #001000;
                border-width: 2px;
                border-color: #001000;
                height: 60px;
                Text {
                    text: root.display_text;
                    font-size: 34px;
                    width: 90%;
                    color: #ffffff;
                    horizontal-alignment: right;
                    vertical-alignment: center;
                    // padding-right: 10px;
                }
            }

            // 按钮网格
            GridLayout {
                spacing: 5px;
                Row {
                    Button { text: "AC"; clicked => {debug("Clear clicked"); root.clearly();}}
                    Button { text: "+/-"; clicked => {debug("Sign/Unsign clicked");}}
                    Button { text: "%"; clicked => {debug("Percent clicked");}}
                    Button { text: "/"; clicked => {debug("Divide clicked"); root.operation("/");}}
                }
                Row {
                    Button { text: "7"; clicked => {root.append_digit("7");debug("7 clicked");}}
                    Button { text: "8"; clicked => {debug("8 clicked");root.append_digit("8");}}
                    Button { text: "9"; clicked => {debug("9 clicked");root.append_digit("9");}}
                    Button { text: "x"; clicked => {debug("Multiply clicked");root.operation("x");}}
                }
                Row {
                    Button { text: "4"; clicked => {debug("4 clicked");root.append_digit("4");}}
                    Button { text: "5"; clicked => {debug("5 clicked");root.append_digit("5");}}
                    Button { text: "6"; clicked => {debug("6 clicked");root.append_digit("6");}}
                    Button { text: "+"; clicked => {debug("Add clicked");root.operation("+");}}
                }
                Row {
                    Button { text: "1"; clicked => {debug("1 clicked");root.append_digit("1");}}
                    Button { text: "2"; clicked => {debug("2 clicked");root.append_digit("2");}}
                    Button { text: "3"; clicked => {debug("3 clicked");root.append_digit("3");}}
                    Button { text: "-"; clicked => {debug("Substract clicked");root.operation("-");}}
                    // 占位
                    // Rectangle {}
                }
                Row {
                    Button { text: "0"; colspan: 2; clicked => { debug("0 clicked"); root.append_digit("0");} }
                    // 空占位符
                    Rectangle {}
                    Button { text: "."; clicked => { debug("Dot clicked"); root.append_dot()} }
                    Button { text: "="; clicked => { debug("Equals clicked"); root.get_result();} }
                }
            }
        }
    }
}

#[derive(Default, Clone)]
struct CalculatorState {
    display_text: String,
    current_value: f64,
    pending_operation: Option<String>,
    waiting_for_operand: bool,
}

fn main() {
    let cal = Calculator::new().unwrap();

    let state = Rc::new(RefCell::new(CalculatorState {
        display_text: "0".to_string(),
        current_value: 0.0,
        pending_operation: None,
        waiting_for_operand: false,
    }));

    {
        let cal_weak = cal.as_weak();
        let state = state.clone();
        // 实现 append_digit 回调
        cal.on_append_digit(move |digit| {
            let mut state = state.borrow_mut();
            if let Some(cal) = cal_weak.upgrade() {
                let new_text = if state.display_text == "0" || state.waiting_for_operand {
                    state.waiting_for_operand = false;
                    digit.to_string()
                } else {
                    format!("{}{}", state.display_text, digit)
                };
                state.display_text = new_text.clone();
                let _ = cal.set_display_text(new_text.into());
            }
        });
    }

    {
        let cal_weak = cal.as_weak();
        let state = state.clone();
        // append_dot 回调实现
        cal.on_append_dot(move || {
            let mut state = state.borrow_mut();
            let current_text = state.display_text.clone();
            if let Some(cal) = cal_weak.upgrade() {
                let new_text = if state.waiting_for_operand {
                    state.waiting_for_operand = false;
                    state.display_text = "0.".to_string();
                    format!("{}", state.display_text)
                } else if !state.display_text.contains(".") {
                    state.display_text.push('.');
                    format!("{}", state.display_text)
                } else {
                    current_text
                };
                let _ = cal.set_display_text(new_text.into());
            }
        });
    }
    // 记录操作符号
    {
        let cal_weak = cal.as_weak();
        let state = state.clone();
        cal.on_operation(move |op| {
            let mut state = state.borrow_mut();
            if let Ok(val) = state.display_text.parse::<f64>() {
                state.pending_operation = Some(op.to_string());
                state.current_value = val;
                state.waiting_for_operand = true;
            }
            if let Some(cal) = cal_weak.upgrade() {
                let _ = cal.set_display_text(state.display_text.clone().into());
            }
        });
    }
    // 计算结果
    {
        let cal_weak = cal.as_weak();
        let state = state.clone();
        cal.on_get_result(move || {
            let mut state = state.borrow_mut();
            if let Some(opt) = &state.pending_operation {
                let current_num = state.display_text.parse::<f64>();
                if let Ok(current_num) = current_num {
                    let result = match opt.as_str() {
                        "+" => state.current_value + current_num,
                        "-" => state.current_value - current_num,
                        "x" => state.current_value * current_num,
                        "/" => {
                            if current_num == 0.0 {
                                f64::NAN
                            } else {
                                state.current_value / current_num
                            }
                        }
                        _ => current_num,
                    };
                    state.current_value = result;
                    state.display_text = result.to_string();
                    state.pending_operation = None;
                    state.waiting_for_operand = true;

                    if let Some(cal) = cal_weak.upgrade() {
                        let _ = cal.set_display_text(state.display_text.clone().into());
                    }
                }
            }
        });
    }
    // 清除回调实现
    {
        let cal_weak = cal.as_weak();
        let state = state.clone();
        cal.on_clearly(move || {
            let mut state = state.borrow_mut();
            if let Some(cal) = cal_weak.upgrade() {
                state.display_text = "0".to_string();
                state.current_value = 0.0;
                state.pending_operation = None;
                state.waiting_for_operand = false;
                let _ = cal.set_display_text("0".into());
            }
        });
    }

    cal.run().unwrap();
}
