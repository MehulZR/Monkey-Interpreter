use crate::{
    ast::{ExpressionStatement, Program, Statement, EXPRESSION},
    object::{Boolean, Integer, Null, Object},
};

// static TRUE: Object = Object::BOOLEAN(Boolean { value: true });
// static FALSE: Object = Object::BOOLEAN(Boolean { value: false });
// static NULL: Object = Object::NULL(Null {});

fn eval(program: Program) -> Vec<Object> {
    let mut result = vec![];
    for stmt in program.statements {
        result.push(eval_statement(stmt));
    }
    result
}

fn eval_statement(stmt: Statement) -> Object {
    match stmt {
        // Statement::LETSTATEMENT(s) => eval_let_statement(s),
        // Statement::RETURNSTATEMENT(s) => eval_return_statement(s),
        Statement::EXPRESSIONSTATEMENT(s) => eval_expression_statement(s),
        other => panic!("no eval fn for stmt of type {:?}", other),
    }
}

// fn eval_let_statement(stmt: LetStatement) -> Object {}
// fn eval_return_statement(stmt: ReturnStatement) -> Object {}
fn eval_expression_statement(stmt: ExpressionStatement) -> Object {
    eval_expression(stmt.expression)
}

fn eval_expression(exp: EXPRESSION) -> Object {
    match exp {
        EXPRESSION::INTEGER(e) => Object::INTEGER(Integer { value: e.value }),
        EXPRESSION::BOOLEAN(e) => Object::BOOLEAN(Boolean { value: e.value }),
        EXPRESSION::PREFIX(e) => {
            let right = eval_expression(*e.right);
            eval_prefix_expression(e.operator, right)
        }
        EXPRESSION::INFIX(e) => {
            let left = eval_expression(*e.left);
            let right = eval_expression(*e.right);
            eval_infix_expression(e.operator, left, right)
        }
        other => panic!("no eval fn for expression of type {:?}", other),
    }
}

fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minux_operator_expression(right),
        _ => Object::NULL(Null {}),
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
            _ => Object::NULL(Null {}),
        },
        (Object::BOOLEAN(obj1), Object::BOOLEAN(obj2)) => match operator.as_str() {
            "!=" => Object::BOOLEAN(Boolean {
                value: obj1.value != obj2.value,
            }),
            "==" => Object::BOOLEAN(Boolean {
                value: obj1.value == obj2.value,
            }),
            _ => Object::NULL(Null {}),
        },
        _ => Object::NULL(Null {}),
    }
}

fn eval_bang_operator_expression(object: Object) -> Object {
    match object {
        Object::BOOLEAN(obj) => Object::BOOLEAN(Boolean { value: !obj.value }),
        Object::NULL(_) => Object::BOOLEAN(Boolean { value: true }),
        _ => Object::BOOLEAN(Boolean { value: false }),
    }
}
fn eval_minux_operator_expression(object: Object) -> Object {
    match object {
        Object::INTEGER(obj) => Object::INTEGER(Integer { value: -obj.value }),
        _ => Object::NULL(Null {}),
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

    fn test_eval(input: String) -> Object {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = match p.parse_program() {
            Some(p) => p,
            None => panic!("error while parsing program"),
        };

        eval(program)[0].clone()
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
