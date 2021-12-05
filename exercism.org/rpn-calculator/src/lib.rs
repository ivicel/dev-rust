#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut buf: Vec<i32> = Vec::new();
    for v in inputs.iter() {
        match v {
            CalculatorInput::Value(i) => buf.push(*i),
            sign => {
                if buf.is_empty() {
                    return None;
                }
                if let Some(v2) = buf.pop() {
                    if let Some(v1) = buf.pop() {
                        let result = match sign {
                            CalculatorInput::Add => v1 + v2,
                            CalculatorInput::Divide => v1 / v2,
                            CalculatorInput::Multiply => v1 * v2,
                            CalculatorInput::Subtract => v1 - v2,
                            CalculatorInput::Value(ref i) => *i,
                        };
                        buf.push(result);
                        continue;
                    }
                }
                return None;
            }
        }
    }

    if buf.len() > 1 {
        None
    } else {
        buf.pop()
    }
}
