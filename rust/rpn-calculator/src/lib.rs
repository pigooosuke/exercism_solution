#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for token in inputs {
        match token {
            CalculatorInput::Add => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b + a);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b - a);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b * a);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b / a);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            CalculatorInput::Value(val) => stack.push(*val),
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
 