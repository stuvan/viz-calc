use eval::eval;
use vizia::prelude::*;

#[derive(Lens)]

pub struct Output {
    expression: String,
    result: String,
    is_decimal_used: bool,
    is_positive_sign: bool,
    is_showing_result: bool,
}

pub enum Operation {
    AllClear,
    ChangeSign,
    Modulo,
    Division,
    Multiplication,
    Subtraction,
    Addition,
    Equal,
    Decimal,
}

pub enum AppEvents {
    Input(i32),
    Operation(Operation),
}

impl Model for Output {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            AppEvents::Input(user_input) => {
                if self.expression.len() == 0 && *user_input == 0 {
                } else {
                    self.expression += &user_input.to_string();
                }
            }
            AppEvents::Operation(op) => handle_app_opearation(self, op),
        });
    }
}

fn handle_app_opearation(data: &mut Output, op: &Operation) {
    match op {
        Operation::AllClear => {
            data.is_decimal_used = false;
            data.is_positive_sign = true;
            data.expression.clear();
        }
        Operation::ChangeSign => {
            if data.is_positive_sign && data.expression.len() != 0 {
                data.expression = format!("-{}", data.expression);
                data.is_positive_sign = false;
            } else if data.expression.len() != 0 {
                data.expression.remove(0);
                data.is_positive_sign = true;
            }
        }
        Operation::Decimal => {
            if !data.is_decimal_used && data.expression.len() != 0 {
                data.expression.push('.');
                data.is_decimal_used = true;
            } else if !data.is_decimal_used && data.expression.len() == 0 {
                data.expression.push_str("0.");
                data.is_decimal_used = true;
            }
        }
        Operation::Addition => {
            let last_character = data.expression.chars().last();
            if data.is_showing_result {
                data.expression = data.result.clone();
                data.is_showing_result = false;
            }
            if last_character != None && last_character != Some('+') {
                if last_character == Some('-')
                    || last_character == Some('*')
                    || last_character == Some('/')
                    || last_character == Some('%')
                {
                    data.expression.pop();
                    data.expression += "+";
                } else {
                    data.expression.push('+');
                }
            }
        }
        Operation::Subtraction => {
            if data.is_showing_result {
                data.expression = data.result.clone();
                data.is_showing_result = false;
            }

            let last_character = data.expression.chars().last();
            if last_character != None && last_character != Some('-') {
                if last_character == Some('+')
                    || last_character == Some('*')
                    || last_character == Some('/')
                    || last_character == Some('%')
                {
                    data.expression.pop();
                    data.expression += "-";
                } else {
                    data.expression.push('-');
                }
            }
        }
        Operation::Multiplication => {
            let last_character = data.expression.chars().last();
            if data.is_showing_result {
                data.expression = data.result.clone();
                data.is_showing_result = false;
            }
            if last_character != None && last_character != Some('*') {
                if last_character == Some('+')
                    || last_character == Some('-')
                    || last_character == Some('/')
                    || last_character == Some('%')
                {
                    data.expression.pop();
                    data.expression += "*";
                } else {
                    data.expression.push('*');
                }
            }
        }
        Operation::Division => {
            let last_character = data.expression.chars().last();
            if data.is_showing_result {
                data.expression = data.result.clone();
                data.is_showing_result = false;
            }
            if last_character != None && last_character != Some('/') {
                if last_character == Some('+')
                    || last_character == Some('-')
                    || last_character == Some('*')
                    || last_character == Some('%')
                {
                    data.expression.pop();
                    data.expression += "/";
                } else {
                    data.expression.push('/');
                }
            }
        }
        Operation::Modulo => {
            let last_character = data.expression.chars().last();
            if data.is_showing_result {
                data.expression = data.result.clone();
                data.is_showing_result = false;
            }
            if last_character != None && last_character != Some('%') {
                if last_character == Some('+')
                    || last_character == Some('-')
                    || last_character == Some('*')
                    || last_character == Some('/')
                {
                    data.expression.pop();
                    data.expression += "%";
                } else {
                    data.expression.push('%');
                }
            }
        }
        Operation::Equal => {
            data.result = eval(&data.expression).unwrap_or_default().to_string();
            data.is_showing_result = true;
        }
    };
}

fn main() {
    Application::new(|cx| {
        Output {
            expression: String::new(),
            result: String::new(),
            is_decimal_used: false,
            is_positive_sign: true,
            is_showing_result: false,
        }
        .build(cx);

        cx.add_stylesheet(include_style!("style/style.css"))
            .expect("Failed to load the stylesheet");

        // The Container
        VStack::new(cx, |cx| {
            // The output screen
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, Output::expression);
                })
                .class("display-expression")
                .height(Percentage(80.0));
                VStack::new(cx, |cx| {
                    Label::new(cx, Output::result);
                })
                .class("display-result");
            })
            .class("display");

            // The Keypad
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::AllClear)),
                            |cx| Label::new(cx, "AC"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::ChangeSign)),
                            |cx| Label::new(cx, "±"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Modulo)),
                            |cx| Label::new(cx, "%"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Division)),
                            |cx| Label::new(cx, "/"),
                        )
                        .class("accent");
                    })
                    .class("button")
                    .class("division");
                })
                .class("keypad-row");
                HStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(7)),
                            |cx| Label::new(cx, "7"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(8)),
                            |cx| Label::new(cx, "8"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(9)),
                            |cx| Label::new(cx, "9"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Multiplication)),
                            |cx| Label::new(cx, "X"),
                        )
                        .class("accent");
                    })
                    .class("multiplication")
                    .class("button");
                })
                .class("keypad-row");
                HStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(4)),
                            |cx| Label::new(cx, "4"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(5)),
                            |cx| Label::new(cx, "5"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(6)),
                            |cx| Label::new(cx, "6"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Subtraction)),
                            |cx| Label::new(cx, "-"),
                        )
                        .class("accent");
                    })
                    .class("button")
                    .class("subtraction");
                })
                .class("keypad-row");
                HStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(1)),
                            |cx| Label::new(cx, "1"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(2)),
                            |cx| Label::new(cx, "2"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(3)),
                            |cx| Label::new(cx, "3"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Addition)),
                            |cx| Label::new(cx, "+"),
                        )
                        .class("accent");
                    })
                    .class("button")
                    .class("addition");
                })
                .class("keypad-row");
                HStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Decimal)),
                            |cx| Label::new(cx, "."),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Input(0)),
                            |cx| Label::new(cx, "0"),
                        )
                        .class("accent");
                    })
                    .class("button");
                    HStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(AppEvents::Operation(Operation::Equal)),
                            |cx| Label::new(cx, "="),
                        )
                        .class("accent");
                    })
                    .class("button")
                    .class("button-span-2")
                    .class("result");
                })
                .class("keypad-row");
            })
            .row_between(Pixels(0.0))
            .class("keypad");
        })
        .class("root");
    })
    .min_inner_size(Some((320, 500)))
    .max_inner_size(Some((320, 500)))
    .title("calculator")
    .resizable(false)
    .run();
}
