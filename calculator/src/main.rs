use slint::slint;
use std::cell::RefCell;
use std::rc::Rc;

slint! {
    import { Button, VerticalBox, HorizontalBox } from "std-widgets.slint";

    export component Calculator inherits Window {
        width: 300px;
        height: 400px;
        background: #001000;

        callback append_digit(string);
        callback append_dot();
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
                    Button { text: "AC"; clicked => {debug("Clear clicked");}}
                    Button { text: "+/-"; clicked => {debug("Sign/Unsign clicked");}}
                    Button { text: "%"; clicked => {debug("Percent clicked");}}
                    Button { text: "/"; clicked => {debug("Divide clicked");}}
                }
                Row {
                    Button { text: "7"; clicked => {root.append_digit("7");debug("7 clicked");}}
                    Button { text: "8"; clicked => {debug("8 clicked");root.append_digit("8");}}
                    Button { text: "9"; clicked => {debug("9 clicked");root.append_digit("9");}}
                    Button { text: "x"; clicked => {debug("Multiply clicked");}}
                }
                Row {
                    Button { text: "4"; clicked => {debug("4 clicked");root.append_digit("4");}}
                    Button { text: "5"; clicked => {debug("5 clicked");root.append_digit("5");}}
                    Button { text: "6"; clicked => {debug("6 clicked");root.append_digit("6");}}
                    Button { text: "+"; clicked => {debug("Add clicked");}}
                }
                Row {
                    Button { text: "1"; clicked => {debug("1 clicked");root.append_digit("1");}}
                    Button { text: "2"; clicked => {debug("2 clicked");root.append_digit("2");}}
                    Button { text: "3"; clicked => {debug("3 clicked");root.append_digit("3");}}
                    Button { text: "-"; clicked => {debug("Substract clicked");}}
                    // 占位
                    // Rectangle {}
                }
                Row {
                    Button { text: "0"; colspan: 2; clicked => { debug("0 clicked"); root.append_digit("0");} }
                    // 空占位符
                    Rectangle {}
                    Button { text: "."; clicked => { debug("Dot clicked"); root.append_dot()} }
                    Button { text: "="; clicked => { debug("Equals clicked"); } }
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

    // append_dot 回调实现
    // cal.on_append_dot(move || {
    //     let current_text = cal.get_display_text();
    //     if !current_text.as_str().contains(".") {
    //         format!("{}.", current_text).into();
    //     } else {
    //         current_text;
    //     }
    // });
    cal.run().unwrap();
}
