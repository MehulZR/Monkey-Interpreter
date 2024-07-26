use crate::{
    ast::{
        BlockStatement, ExpressionStatement, IfExpression, Program, ReturnStatement, Statement,
        EXPRESSION,
    },
    object::{Boolean, Error, Integer, Null, Object, Return},
};

fn eval(program: Program) -> Object {
    eval_statements(program.statements)
}

fn eval_statements(stmts: Vec<Statement>) -> Object {
    let mut result = Object::NULL(Null {});

    for stmt in stmts {
        result = eval_statement(stmt);

        match result {
            Object::RETURN(o) => return *o.value,
            Object::ERROR(_) => return result,
            _ => continue,
        }
    }

    result
}

fn eval_statement(stmt: Statement) -> Object {
    match stmt {
        // Statement::LETSTATEMENT(s) => eval_let_statement(s),
        Statement::RETURNSTATEMENT(s) => eval_return_statement(s),
        Statement::EXPRESSIONSTATEMENT(s) => eval_expression_statement(s),
        other => panic!("no eval fn for stmt of type {:?}", other),
    }
}

// fn eval_let_statement(stmt: LetStatement) -> Object {}
fn eval_return_statement(stmt: ReturnStatement) -> Object {
    let val = eval_expression(stmt.return_value);

    if is_error(&val) {
        return val;
    }

    Object::RETURN(Return {
        value: Box::new(val),
    })
}
fn eval_expression_statement(stmt: ExpressionStatement) -> Object {
    eval_expression(stmt.expression)
}

fn eval_expression(exp: EXPRESSION) -> Object {
    match exp {
        EXPRESSION::INTEGER(e) => Object::INTEGER(Integer { value: e.value }),
        EXPRESSION::BOOLEAN(e) => Object::BOOLEAN(Boolean { value: e.value }),
        EXPRESSION::PREFIX(e) => {
            let right = eval_expression(*e.right);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(e.operator, right)
        }
        EXPRESSION::INFIX(e) => {
            let left = eval_expression(*e.left);
            if is_error(&left) {
                return left;
            }
            let right = eval_expression(*e.right);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(e.operator, left, right)
        }
        EXPRESSION::IF(e) => eval_if_expression(e),
        other => panic!("no eval fn for expression of type {:?}", other),
    }
}

fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Object::ERROR(Error {
            msg: format!("unknown operator: {} {:?}", operator, right),
        }),
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::INTEGER(obj1), Object::INTEGER(obj2)) => match operator.as_str() {
            "+" => Object::INTEGER(Integer {
                value: obj1.value + obj2.value,
            }),
            "-" => Object::INTEGER(Integer {
                value: obj1.value - obj2.value,
            }),
            "*" => Object::INTEGER(Integer {
                value: obj1.value * obj2.value,
            }),
            "/" => Object::INTEGER(Integer {
                value: obj1.value / obj2.value,
            }),
            "<" => Object::BOOLEAN(Boolean {
                value: obj1.value < obj2.value,
            }),
            ">" => Object::BOOLEAN(Boolean {
                value: obj1.value > obj2.value,
            }),
            "!=" => Object::BOOLEAN(Boolean {
                value: obj1.value != obj2.value,
            }),
            "==" => Object::BOOLEAN(Boolean {
                value: obj1.value == obj2.value,
            }),
            other => Object::ERROR(Error {
                msg: format!("unknown operator {:?} {} {:?}", obj1, other, obj2),
            }),
        },
        (Object::BOOLEAN(obj1), Object::BOOLEAN(obj2)) => match operator.as_str() {
            "!=" => Object::BOOLEAN(Boolean {
                value: obj1.value != obj2.value,
            }),
            "==" => Object::BOOLEAN(Boolean {
                value: obj1.value == obj2.value,
            }),
            other => Object::ERROR(Error {
                msg: format!("unknown operator {:?} {} {:?}", obj1, other, obj2),
            }),
        },
        (l, r) => Object::ERROR(Error {
            msg: format!("type mismatch {:?} {} {:?}", l, operator, r),
        }),
    }
}

fn eval_bang_operator_expression(object: Object) -> Object {
    match object {
        Object::BOOLEAN(obj) => Object::BOOLEAN(Boolean { value: !obj.value }),
        Object::NULL(_) => Object::BOOLEAN(Boolean { value: true }),
        _ => Object::BOOLEAN(Boolean { value: false }),
    }
}
fn eval_minus_operator_expression(object: Object) -> Object {
    match object {
        Object::INTEGER(obj) => Object::INTEGER(Integer { value: -obj.value }),
        _ => Object::ERROR(Error {
            msg: format!("unknown operator -{:?}", object),
        }),
    }
}

fn eval_if_expression(exp: IfExpression) -> Object {
    let condition = eval_expression(*exp.condition);

    if is_error(&condition) {
        return condition;
    }

    if is_truthy(condition) {
        // return eval_statements(exp.consequence.statements);
        return eval_block_statements(exp.consequence);
    }

    match exp.alternative {
        // Some(alt) => eval_statements(alt.statements),
        Some(alt) => eval_block_statements(alt),
        None => Object::NULL(Null {}),
    }
}

fn eval_block_statements(block_stmt: BlockStatement) -> Object {
    let mut result = Object::NULL(Null {});

    for stmt in block_stmt.statements {
        result = eval_statement(stmt);

        match result {
            Object::RETURN(_) => return result,
            Object::ERROR(_) => return result,
            _ => continue,
        }
    }

    result
}
fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::NULL(_) => false,
        Object::BOOLEAN(o) => o.value,
        _ => true,
    }
}

fn is_error(obj: &Object) -> bool {
    match obj {
        Object::ERROR(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::eval;

    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        struct Test {
            input: String,
            expected: i64,
        }

        let tests = [
            Test {
                input: "5".to_string(),
                expected: 5,
            },
            Test {
                input: "10".to_string(),
                expected: 10,
            },
            Test {
                input: "-5".to_string(),
                expected: -5,
            },
            Test {
                input: "-10".to_string(),
                expected: -10,
            },
            Test {
                input: "5 + 5 + 5 + 5 - 10".to_string(),
                expected: 10,
            },
            Test {
                input: "2 * 2 * 2 * 2 * 2".to_string(),
                expected: 32,
            },
            Test {
                input: "-50 + 100 + -50".to_string(),
                expected: 0,
            },
            Test {
                input: "5 * 2 + 10".to_string(),
                expected: 20,
            },
            Test {
                input: "5 + 2 * 10".to_string(),
                expected: 25,
            },
            Test {
                input: "20 + 2 * -10".to_string(),
                expected: 0,
            },
            Test {
                input: "50 / 2 * 2 + 10".to_string(),
                expected: 60,
            },
            Test {
                input: "2 * (5 + 10)".to_string(),
                expected: 30,
            },
            Test {
                input: "3 * 3 * 3 + 10".to_string(),
                expected: 37,
            },
            Test {
                input: "3 * (3 * 3) + 10".to_string(),
                expected: 37,
            },
            Test {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10".to_string(),
                expected: 50,
            },
        ];

        for test in tests {
            let evaluated_value = test_eval(test.input.clone());
            test_integer_object(evaluated_value, test.expected);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        struct Test {
            input: String,
            expected: bool,
        }

        let tests = [
            Test {
                input: "true".to_string(),
                expected: true,
            },
            Test {
                input: "false".to_string(),
                expected: false,
            },
            Test {
                input: "1 < 2".to_string(),
                expected: true,
            },
            Test {
                input: "1 > 2".to_string(),
                expected: false,
            },
            Test {
                input: "1 < 1".to_string(),
                expected: false,
            },
            Test {
                input: "1 > 1".to_string(),
                expected: false,
            },
            Test {
                input: "1 == 1".to_string(),
                expected: true,
            },
            Test {
                input: "1 != 1".to_string(),
                expected: false,
            },
            Test {
                input: "1 == 2".to_string(),
                expected: false,
            },
            Test {
                input: "1 != 2".to_string(),
                expected: true,
            },
            Test {
                input: "true == true".to_string(),
                expected: true,
            },
            Test {
                input: "false == false".to_string(),
                expected: true,
            },
            Test {
                input: "true == false".to_string(),
                expected: false,
            },
            Test {
                input: "true != false".to_string(),
                expected: true,
            },
            Test {
                input: "false != true".to_string(),
                expected: true,
            },
            Test {
                input: "(1 < 2) == true".to_string(),
                expected: true,
            },
            Test {
                input: "(1 < 2) == false".to_string(),
                expected: false,
            },
            Test {
                input: "(1 > 2) == true".to_string(),
                expected: false,
            },
            Test {
                input: "(1 > 2) == false".to_string(),
                expected: true,
            },
        ];

        for test in tests {
            let evaluated_value = test_eval(test.input.clone());
            test_boolean_object(evaluated_value, test.expected);
        }
    }

    #[test]
    fn test_bang_operator() {
        struct Test {
            input: String,
            expected: bool,
        }

        let tests = [
            Test {
                input: "!true".to_string(),
                expected: false,
            },
            Test {
                input: "!false".to_string(),
                expected: true,
            },
            Test {
                input: "!5".to_string(),
                expected: false,
            },
            Test {
                input: "!!true".to_string(),
                expected: true,
            },
            Test {
                input: "!!false".to_string(),
                expected: false,
            },
            Test {
                input: "!!5".to_string(),
                expected: true,
            },
        ];

        for test in tests {
            let evaluated_val = test_eval(test.input);
            test_boolean_object(evaluated_val, test.expected);
        }
    }

    #[test]
    fn test_if_else_expression() {
        struct Test {
            input: String,
            expected: Option<i64>,
        }

        let tests = [
            Test {
                input: "if (true) { 10 }".to_string(),
                expected: Some(10),
            },
            Test {
                input: "if (false) { 10 }".to_string(),
                expected: None,
            },
            Test {
                input: "if (1) { 10 }".to_string(),
                expected: Some(10),
            },
            Test {
                input: "if (1 < 2) { 10 }".to_string(),
                expected: Some(10),
            },
            Test {
                input: "if (1 > 2) { 10 }".to_string(),
                expected: None,
            },
            Test {
                input: "if (1 > 2) { 10 } else { 20 }".to_string(),
                expected: Some(20),
            },
            Test {
                input: "if (1 < 2) { 10 } else { 20 }".to_string(),
                expected: Some(10),
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input.clone());
            match test.expected {
                Some(num) => test_integer_object(evaluated, num),
                None => test_null_object(evaluated),
            }
        }
    }

    #[test]
    fn test_return_statements() {
        struct Test {
            input: String,
            expected: i64,
        }

        let tests = [
            Test {
                input: "return 10;".to_string(),
                expected: 10,
            },
            Test {
                input: "return 10; 9;".to_string(),
                expected: 10,
            },
            Test {
                input: "return 2 * 5; 9;".to_string(),
                expected: 10,
            },
            Test {
                input: "9; return 2 * 5; 9;".to_string(),
                expected: 10,
            },
            Test {
                input: "if (10 > 1) { if (10 > 1) { return 10; } return 1; }".to_string(),
                expected: 10,
            },
        ];

        for test in tests {
            let evaluated_val = test_eval(test.input);
            test_integer_object(evaluated_val, test.expected);
        }
    }

    #[test]
    fn test_error_heandling() {
        struct Test {
            input: String,
            expected: String,
        }

        let tests = [
            Test {
                input: "5 + true;".to_string(),
                expected:
                    "type mismatch INTEGER(Integer { value: 5 }) + BOOLEAN(Boolean { value: true })"
                        .to_string(),
            },
            Test {
                input: "5 + true; 5;".to_string(),
                expected:
                    "type mismatch INTEGER(Integer { value: 5 }) + BOOLEAN(Boolean { value: true })"
                        .to_string(),
            },
            Test {
                input: "-true".to_string(),
                expected: "unknown operator -BOOLEAN(Boolean { value: true })".to_string(),
            },
            Test {
                input: "true + false;".to_string(),
                expected: "unknown operator Boolean { value: true } + Boolean { value: false }"
                    .to_string(),
            },
            Test {
                input: "5; true + false; 5".to_string(),
                expected: "unknown operator Boolean { value: true } + Boolean { value: false }"
                    .to_string(),
            },
            Test {
                input: "if (10 > 1) { true + false; }".to_string(),
                expected: "unknown operator Boolean { value: true } + Boolean { value: false }"
                    .to_string(),
            },
            Test {
                input: "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }".to_string(),
                expected: "unknown operator Boolean { value: true } + Boolean { value: false }"
                    .to_string(),
            },
        ];

        for test in tests {
            let evaluated_val = test_eval(test.input);
            match evaluated_val {
                Object::ERROR(obj) => {
                    if obj.msg != test.expected {
                        panic!(
                            "wrong error msg. Expected {} got {}",
                            test.expected, obj.msg
                        )
                    }
                }
                other => panic!("no error object returned. Got {:?}", other),
            }
        }
    }

    fn test_null_object(obj: Object) {
        match obj {
            Object::NULL(_) => {}
            other => panic!("Object is not of type null. Got {:?}", other),
        }
    }

    fn test_eval(input: String) -> Object {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = match p.parse_program() {
            Some(p) => p,
            None => panic!("error while parsing program"),
        };

        eval(program).clone()
    }

    fn test_integer_object(object: Object, expected: i64) {
        match object {
            Object::INTEGER(obj) => {
                if obj.value != expected {
                    panic!(
                        "Object has wrong value. Got {}, want {}",
                        obj.value, expected
                    )
                }
            }
            other => panic!("Object is not of type Integer. Got {:?}", other),
        }
    }

    fn test_boolean_object(object: Object, expected: bool) {
        match object {
            Object::BOOLEAN(obj) => {
                if obj.value != expected {
                    panic!(
                        "Object has wrong value. Got {}, want {}",
                        obj.value, expected
                    )
                }
            }
            other => panic!("Object is not of type Boolean. Got {:?}", other),
        }
    }
}
