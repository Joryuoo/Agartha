use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression};
use crate::token::Token; 
use std::io::{self, Write}; 

pub struct Interpreter {
    memory: HashMap<String, Expression>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { memory: HashMap::new() }
    }

    pub fn interpret(&mut self, program: Program) {
        for stmt in program.statements {
            if let Err(e) = self.execute_statement(stmt) {
                println!("Runtime Error: {}", e);
                break; 
            }
        }
    }

    // STATEMENT EXECUTOR
    fn execute_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            // Declarations (NUMBER n = 10;)
            Statement::Declaration { name, value, .. } => {
                if let Some(expr) = value {
                    let final_val = self.evaluate_expression(expr)?;
                    self.memory.insert(name, final_val);
                }
                Ok(())
            }

            // Assignments (n = 20;)
            Statement::Assignment { name, value } => {
                if !self.memory.contains_key(&name) {
                    return Err(format!("Unsa man nang '{}'? Wa na gi-declare dong!", name));
                }
                let final_val = self.evaluate_expression(value)?;
                self.memory.insert(name, final_val);
                Ok(())
            }

            // Input (input(n);)
            Statement::Input(name) => {
                io::stdout().flush().unwrap(); 
                let mut input_text = String::new();
                io::stdin().read_line(&mut input_text).unwrap();
                let trimmed = input_text.trim();

                // Auto-detect what type of data the user typed
                let expr = if let Ok(n) = trimmed.parse::<i32>() {
                    Expression::Number(n)
                } else if let Ok(d) = trimmed.parse::<f64>() {
                    Expression::Decimal(d)
                } else {
                    Expression::Word(trimmed.to_string())
                };

                self.memory.insert(name, expr);
                Ok(())
            }

            // Display (display("hello");)
            Statement::Display(expr) => {
                let val = self.evaluate_expression(expr)?;
                // stringify 
                println!("{}", self.stringify(val)); 
                Ok(())
            }

            _ => Err("This statement is not supported in the Interpreter yet!".to_string()) 
        }
    }

    // --- EXPRESSION CALCULATOR ---
    fn evaluate_expression(&mut self, expr: Expression) -> Result<Expression, String> {
        match expr {
            Expression::Number(n) => Ok(Expression::Number(n)),
            Expression::Word(w) => Ok(Expression::Word(w)),
            Expression::Decimal(d) => Ok(Expression::Decimal(d)),
            Expression::Truth(t) => Ok(Expression::Truth(t)),
            Expression::Letter(l) => Ok(Expression::Letter(l)),

            Expression::Identifier(name) => {
                if let Some(val) = self.memory.get(&name) {
                    Ok(val.clone())
                } else {
                    Err(format!("Variable '{}' not found!", name))
                }
            }

            // 5. Binary Operations (Math and Concat)
            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;

                match operator {
                    // CONCATENATION (&)
                    Token::Concat => {
                        let l_str = self.stringify(left_val);
                        let r_str = self.stringify(right_val);
                        Ok(Expression::Word(format!("{}{}", l_str, r_str)))
                    },
                    
                    // ADDITION (+)
                    Token::Add => {
                        match (left_val, right_val) {
                            (Expression::Number(l), Expression::Number(r)) => Ok(Expression::Number(l + r)),
                            (Expression::Decimal(l), Expression::Decimal(r)) => Ok(Expression::Decimal(l + r)),
                            _ => Err("Math Error: Can only add numbers together.".to_string())
                        }
                    },

                    _ => Err(format!("Operator {:?} is not fully implemented yet!", operator))
                }
            }
            _ => Err("This expression is not supported yet!".to_string()),
        }
    }

    // --- HELPER TOOL ---
    fn stringify(&self, expr: Expression) -> String {
        match expr {
            Expression::Number(n) => n.to_string(),
            Expression::Decimal(d) => d.to_string(),
            Expression::Word(w) => w,
            Expression::Truth(t) => t.to_string(),
            Expression::Letter(l) => l.to_string(),
            _ => "".to_string(),
        }
    }
}