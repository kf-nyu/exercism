use std::{collections::HashMap, rc::Rc};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    customs: HashMap<String, Rc<Vec<Op>>>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Push(Value),
    Add,
    Sub,
    Mul,
    Div,
    Drop,
    Swap,
    Dup,
    Over,
    DupTwice,
    Countup,
    UseDefinition(Rc<Vec<Op>>),
}
impl Op {
    pub fn from_token(token: &str) -> std::result::Result<Self, crate::Error> {
        match token {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "drop" => Ok(Op::Drop),
            "swap" => Ok(Op::Swap),
            "dup" => Ok(Op::Dup),
            "over" => Ok(Op::Over),
            "duptwice" => Ok(Op::DupTwice),
            "countup" => Ok(Op::Countup),
            _ => token
                .parse::<Value>()
                .map(Op::Push)
                .map_err(|_| crate::Error::UnknownWord),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            customs: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if input.is_empty() {
            return Ok(());
        }

        let input_lower = input.to_lowercase();
        let items: Vec<&str> = input_lower.split_whitespace().collect();

        if items[0] == ":" {
            if items[items.len() - 1] == ";" {
                if items.len() < 4 || items[1].parse::<Value>().is_ok() {
                    Err(crate::Error::InvalidWord)
                } else {
                    let mut custom_commands: Vec<Op> = Vec::new();
                    for &item in items.iter().take(items.len() - 1).skip(2) {
                        if let Some(existing) = self.customs.get(item) {
                            custom_commands.push(Op::UseDefinition(Rc::clone(existing)));
                        } else {
                            custom_commands.push(Op::from_token(item)?);
                        }
                    }
                    self.customs
                        .insert(items[1].to_string(), Rc::new(custom_commands));
                    Ok(())
                }
            } else {
                Err(crate::Error::InvalidWord)
            }
        } else {
            for item in items {
                if let Some(commands) = self.customs.get(item).cloned() {
                    for op in commands.iter() {
                        self.eval_op(op.clone())?;
                    }
                } else {
                    let op = Op::from_token(item)?;
                    self.eval_op(op)?;
                }
            }
            Ok(())
        }
    }

    pub fn eval_op(&mut self, op: Op) -> Result {
        match op {
            Op::Push(n) => self.stack.push(n),
            Op::Add => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a + b);
            }
            Op::Sub => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a - b);
            }
            Op::Mul => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a * b);
            }
            Op::Div => {
                let (a, b) = self.pop_two()?;
                if b == 0 {
                    return Err(crate::Error::DivisionByZero);
                } else {
                    self.stack.push(a / b);
                }
            }
            Op::Drop => {
                self.stack.pop().ok_or(crate::Error::StackUnderflow)?;
            }
            Op::Dup => {
                let top = *self.stack.last().ok_or(crate::Error::StackUnderflow)?;
                self.stack.push(top);
            }
            Op::Swap => {
                let (a, b) = self.pop_two()?;
                self.stack.push(b);
                self.stack.push(a);
            }
            Op::Over => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a);
                self.stack.push(b);
                self.stack.push(a);
            }
            Op::DupTwice => {
                let top = *self.stack.last().ok_or(crate::Error::StackUnderflow)?;
                self.stack.push(top);
                self.stack.push(top);
            }
            Op::Countup => {
                self.stack.extend([1, 2, 3]);
            }
            Op::UseDefinition(command) => {
                for op in command.iter() {
                    self.eval_op(op.clone())?;
                }
            }
        }
        Ok(())
    }

    fn pop_two(&mut self) -> std::result::Result<(Value, Value), crate::Error> {
        let b = self.stack.pop().ok_or(crate::Error::StackUnderflow)?;
        let a = self.stack.pop().ok_or(crate::Error::StackUnderflow)?;
        Ok((a, b))
    }

}
