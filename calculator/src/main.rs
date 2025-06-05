use slint::slint;

slint! {
    import { Button, VerticalBox, HorizontalBox } from "std-widgets.slint";

    export component App inherits Window {
        width: 300px;
        height: 400px;
        background: #001000;

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
                    text: "0";
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
                    Button { text: "7"; clicked => {debug("7 clicked");}}
                    Button { text: "8"; clicked => {debug("8 clicked");}}
                    Button { text: "9"; clicked => {debug("9 clicked");}}
                    Button { text: "x"; clicked => {debug("Multiply clicked");}}
                }
                Row {
                    Button { text: "4"; clicked => {debug("4 clicked");}}
                    Button { text: "5"; clicked => {debug("5 clicked");}}
                    Button { text: "6"; clicked => {debug("6 clicked");}}
                    Button { text: "+"; clicked => {debug("Add clicked");}}
                }
                Row {
                    Button { text: "1"; clicked => {debug("1 clicked");}}
                    Button { text: "2"; clicked => {debug("2 clicked");}}
                    Button { text: "3"; clicked => {debug("3 clicked");}}
                    Button { text: "-"; clicked => {debug("Substract clicked");}}
                    // 占位
                    // Rectangle {}
                }
                Row {
                    Button { text: "0"; colspan: 2; clicked => { debug("0 clicked"); } }
                    // 空占位符
                    Rectangle {}
                    Button { text: "."; clicked => { debug("Dot clicked"); } }
                    Button { text: "="; clicked => { debug("Equals clicked"); } }
                }
            }

        }
    }
}

fn main() {
    App::new().unwrap().run().unwrap();
}
