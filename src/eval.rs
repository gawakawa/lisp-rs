use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::Object;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    vars: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }
}

pub fn eval(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_, _) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Symbol(s) => eval_symbol(s, env),
        Object::List(list) => eval_list(list, env),
    }
}

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = env.borrow_mut().get(s);
    match val {
        Some(v) => Ok(v.clone()),
        None => Err(format!("Unbound symbol: {}", s)),
    }
}

fn eval_list(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    if let Object::Symbol(s) = head {
        match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_op(list, env),
            "define" => eval_define(list, env),
            "if" => eval_if(list, env),
            "lambda" => eval_function_definition(list),
            _ => eval_function_call(s, list, env),
        }
    } else {
        let new_list: Result<Vec<_>, _> = list
            .iter()
            .map(|obj| eval(obj, env))
            .filter(|result| !matches!(result, Ok(Object::Void)))
            .collect();
        Ok(Object::List(new_list?))
    }
}

fn eval_define(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for define".to_string());
    };

    let sym = if let Object::Symbol(s) = &list[1] {
        s.clone()
    } else {
        return Err("Invalid define".to_string());
    };

    let val = eval(&list[2], env)?;
    env.borrow_mut().set(&sym, val);

    Ok(Object::Void)
}

fn eval_binary_op(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for infix operator".to_string());
    }

    let operator = list[0].clone();
    let left = eval(&list[1].clone(), env)?;
    let right = eval(&list[2].clone(), env)?;

    let left_val = if let Object::Integer(n) = left {
        n
    } else {
        return Err(format!("Left operand must be an integer {:?}", left));
    };
    let right_val = if let Object::Integer(n) = right {
        n
    } else {
        return Err(format!("Right operand must be an integer {:?}", right));
    };

    if let Object::Symbol(s) = operator {
        match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            "<" => Ok(Object::Bool(left_val < right_val)),
            ">" => Ok(Object::Bool(left_val > right_val)),
            "=" => Ok(Object::Bool(left_val == right_val)),
            "!=" => Ok(Object::Bool(left_val != right_val)),
            _ => Err(format!("Invalid infix operator: {}", s)),
        }
    } else {
        Err("Operator must be a symbol".to_string())
    }
}

fn eval_if(list: &[Object], env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err("Invalid number of arguments for if statement".to_string());
    }

    let cond_obj = eval(&list[1], env)?;
    let cond = if let Object::Bool(b) = cond_obj {
        b
    } else {
        return Err("Condition must be boolean".to_string());
    };

    eval(if cond { &list[2] } else { &list[3] }, env)
}

fn eval_function_definition(list: &[Object]) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(list) => list
            .iter()
            .map(|param| match param {
                Object::Symbol(s) => Ok(s.clone()),
                _ => Err("Invalid lambda parameter".to_string()),
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => return Err("Invalid lambda".to_string()),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err("Invalid lambad".to_string()),
    };

    Ok(Object::Lambda(params, body))
}

fn eval_function_call(
    s: &str,
    list: &[Object],
    env: &mut Rc<RefCell<Env>>,
) -> Result<Object, String> {
    let lambda = env
        .borrow_mut()
        .get(s)
        .ok_or_else(|| format!("Unbound symbol: {}", s))?;

    match lambda {
        Object::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }
            eval(&Object::List(body), &mut new_env)
        }
        _ => Err(format!("Not a lambda: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let input = Object::List(vec![
            Object::Symbol("+".to_string()),
            Object::Integer(1),
            Object::Integer(2),
        ]);
        let expeted = Ok(Object::Integer(3));

        assert_eq!(expeted, eval(&input, &mut env));
    }
}
