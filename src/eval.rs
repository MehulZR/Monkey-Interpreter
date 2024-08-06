use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        BlockStatement, CallExpression, ExpressionStatement, HashLiteral, Identifier, IfExpression,
        LetStatement, Program, ReturnStatement, Statement, EXPRESSION,
    },
    object::{
        enclosed_environment, Array, Boolean, Environment, Error, Function, HashObject, HashPair,
        Hashable, Integer, Null, Object, Return, StringLiteral, BUILTINS,
    },
};

fn eval(program: Program) -> Object {
    let env = Rc::new(RefCell::new(Environment::new()));
    eval_statements(program.statements, &env)
}

fn eval_statements(stmts: Vec<Statement>, env: &Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::NULL(Null {});

    for stmt in stmts {
        match eval_statement(stmt, env) {
            Some(r) => result = r,
            None => continue,
        };

        match result {
            Object::RETURN(o) => return *o.value,
            Object::ERROR(_) => return result,
            _ => continue,
        }
    }

    result
}

fn eval_statement(stmt: Statement, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    match stmt {
        Statement::LETSTATEMENT(s) => eval_let_statement(s, env),
        Statement::RETURNSTATEMENT(s) => Some(eval_return_statement(s, env)),
        Statement::EXPRESSIONSTATEMENT(s) => Some(eval_expression_statement(s, env)),
    }
}

fn eval_let_statement(stmt: LetStatement, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    let val = eval_expression(stmt.value, env);

    if is_error(&val) {
        return Some(val);
    }

    env.borrow_mut().set(stmt.name.value, val);

    None
}
fn eval_return_statement(stmt: ReturnStatement, env: &Rc<RefCell<Environment>>) -> Object {
    let val = eval_expression(stmt.return_value, env);

    if is_error(&val) {
        return val;
    }

    Object::RETURN(Return {
        value: Box::new(val),
    })
}
fn eval_expression_statement(stmt: ExpressionStatement, env: &Rc<RefCell<Environment>>) -> Object {
    eval_expression(stmt.expression, env)
}

fn eval_expression(exp: EXPRESSION, env: &Rc<RefCell<Environment>>) -> Object {
    match exp {
        EXPRESSION::INTEGER(e) => Object::INTEGER(Integer { value: e.value }),
        EXPRESSION::BOOLEAN(e) => Object::BOOLEAN(Boolean { value: e.value }),
        EXPRESSION::IF(e) => eval_if_expression(e, env),
        EXPRESSION::IDENTIFIER(e) => eval_identifier(e, env),
        EXPRESSION::FN(e) => Object::FN(Function {
            params: e.parameters,
            body: e.body,
            env: Rc::clone(env),
        }),
        EXPRESSION::CALL(e) => eval_call_expression(e, env),
        EXPRESSION::PREFIX(e) => {
            let right = eval_expression(*e.right, env);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(e.operator, right)
        }
        EXPRESSION::INFIX(e) => {
            let left = eval_expression(*e.left, env);
            if is_error(&left) {
                return left;
            }
            let right = eval_expression(*e.right, env);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(e.operator, left, right)
        }
        EXPRESSION::StringLiteral(e) => Object::STRING(StringLiteral { value: e.value }),
        EXPRESSION::ArrayLiteral(e) => {
            let elements = eval_expressions(e.items, env);

            if elements.len() == 1 && is_error(&elements[0]) {
                return elements[0].clone();
            }

            Object::ARRAY(Array { elements })
        }
        EXPRESSION::IndexExpression(e) => {
            let left = eval_expression(*e.left, env);
            if is_error(&left) {
                return left;
            }

            let index = eval_expression(*e.index, env);
            if is_error(&index) {
                return index;
            }

            eval_index_expression(left, index)
        }
        EXPRESSION::HashLiteral(e) => eval_hash_literal(e, env),
        other => panic!("eval fn not found for expression: {:?}", other),
    }
}

fn eval_hash_literal(hash_lit: HashLiteral, env: &Rc<RefCell<Environment>>) -> Object {
    let mut pairs = HashMap::new();

    for (k, v) in hash_lit.pairs {
        let key = eval_expression(k, env);
        if is_error(&key) {
            return key;
        }

        let hash_key;

        match &key {
            Object::STRING(o) => {
                hash_key = o.hash_key();
            }
            Object::INTEGER(o) => {
                hash_key = o.hash_key();
            }
            Object::BOOLEAN(o) => {
                hash_key = o.hash_key();
            }
            other => {
                return Object::ERROR(Error {
                    msg: format!("unusable as hash key: {:?}", other),
                });
            }
        };

        let value = eval_expression(v, env);
        if is_error(&value) {
            return value;
        }

        pairs.insert(hash_key, HashPair { key, value });
    }

    Object::HashLitearl(HashObject { pairs })
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    match (left, index) {
        (Object::ARRAY(arr), Object::INTEGER(i)) => {
            let max = arr.elements.len();
            let i = match usize::try_from(i.value) {
                Ok(i) => i,
                Err(_) => {
                    return Object::NULL(Null {});
                }
            };

            if i >= max {
                return Object::NULL(Null {});
            }

            arr.elements[i].clone()
        }
        (other, _) => panic!("Index operator not supported on {:?}", other),
    }
}

fn eval_call_expression(call_exp: CallExpression, env: &Rc<RefCell<Environment>>) -> Object {
    let function = eval_expression(*call_exp.function, env);

    if is_error(&function) {
        return function;
    }

    let evaluated_args = eval_expressions(call_exp.args, env);

    if evaluated_args.len() == 1 && is_error(&evaluated_args[0]) {
        return evaluated_args[0].clone();
    }

    match function {
        Object::FN(obj) => {
            let extended_env = extend_fn_env(&obj, evaluated_args);

            let evaluated_function = eval_block_statements(obj.body, &extended_env);

            match evaluated_function {
                Object::RETURN(obj) => *obj.value,
                _ => evaluated_function,
            }
        }
        Object::BUILTINFUNC(obj) => (obj.func)(evaluated_args),
        other => panic!("expected fn object. Got {:?}", other),
    }
}

fn extend_fn_env(function: &Function, args: Vec<Object>) -> Rc<RefCell<Environment>> {
    let mut inner_env = enclosed_environment(&function.env);

    for (i, param) in function.params.iter().enumerate() {
        inner_env.set(param.value.clone(), args[i].clone());
    }

    Rc::new(RefCell::new(inner_env))
}

fn eval_expressions(exps: Vec<EXPRESSION>, env: &Rc<RefCell<Environment>>) -> Vec<Object> {
    let mut result = vec![];

    for exp in exps {
        let evaluated = eval_expression(exp, env);

        if is_error(&evaluated) {
            return vec![evaluated];
        }

        result.push(evaluated);
    }

    result
}

fn eval_identifier(ident: Identifier, env: &Rc<RefCell<Environment>>) -> Object {
    if let Some(obj) = env.borrow().get(&ident.value) {
        return obj.clone();
    };

    if let Some(obj) = BUILTINS.get(&ident.value.as_str()) {
        return Object::BUILTINFUNC(obj.clone());
    };

    Object::ERROR(Error {
        msg: format!("identifier not found: {}", ident.value),
    })
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
        (Object::STRING(obj1), Object::STRING(obj2)) => match operator.as_str() {
            "+" => Object::STRING(StringLiteral {
                value: format!("{}{}", obj1.value, obj2.value),
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

fn eval_if_expression(exp: IfExpression, env: &Rc<RefCell<Environment>>) -> Object {
    let condition = eval_expression(*exp.condition, env);

    if is_error(&condition) {
        return condition;
    }

    if is_truthy(condition) {
        return eval_block_statements(exp.consequence, env);
    }

    match exp.alternative {
        Some(alt) => eval_block_statements(alt, env),
        None => Object::NULL(Null {}),
    }
}

fn eval_block_statements(block_stmt: BlockStatement, env: &Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::NULL(Null {});

    for stmt in block_stmt.statements {
        match eval_statement(stmt, env) {
            Some(r) => result = r,
            None => continue,
        }

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
    use std::collections::HashMap;

    use super::eval;

    use crate::ast::Node;
    use crate::lexer::Lexer;
    use crate::object::{Boolean, Hashable, Integer, Object, ObjectTrait, StringLiteral};
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
    fn test_let_statements() {
        struct Test {
            input: String,
            expected: i64,
        }

        let tests = [
            Test {
                input: "let a = 5; a;".to_string(),
                expected: 5,
            },
            Test {
                input: "let a = 5 * 5; a;".to_string(),
                expected: 25,
            },
            Test {
                input: "let a = 5; let b = a; b;".to_string(),
                expected: 5,
            },
            Test {
                input: "let a = 5; let b = a; let c = a + b + 5; c;".to_string(),
                expected: 15,
            },
        ];

        for test in tests {
            let evaluated_val = test_eval(test.input);
            test_integer_object(evaluated_val, test.expected);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };".to_string();
        let evaluated_val = test_eval(input);

        let func = match evaluated_val {
            Object::FN(o) => o,
            other => panic!("object is not Function. Got {:?}", other),
        };

        if func.params.len() != 1 {
            panic!(
                "function has wrong parameters. Paremeters {:?}",
                func.params
            );
        }

        if func.params[0].string() != "x".to_string() {
            panic!("parameter[0] is not x. Got {:?}", func.params[0]);
        }

        if func.body.string() != "(x + 2)".to_string() {
            panic!("body is not (x + 2). Got {}", func.body.string());
        }
    }

    #[test]
    fn test_function_application() {
        struct Test {
            input: String,
            expected: i64,
        }

        let tests = [
            Test {
                input: "let identity = fn(x) { x; }; identity(5);".to_string(),
                expected: 5,
            },
            Test {
                input: "let identity = fn(x) { return x; }; identity(5);".to_string(),
                expected: 5,
            },
            Test {
                input: "let double = fn(x) { x * 2; }; double(5);".to_string(),
                expected: 10,
            },
            Test {
                input: "let add = fn(x, y) { x + y; }; add(5, 5);".to_string(),
                expected: 10,
            },
            Test {
                input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_string(),
                expected: 20,
            },
            Test {
                input: "fn(x) { x; }(5)".to_string(),
                expected: 5,
            },
        ];

        for test in tests {
            let evaluated_val = test_eval(test.input);
            test_integer_object(evaluated_val, test.expected);
        }
    }

    #[test]
    fn test_closure() {
        let input =
            "let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);"
                .to_string();
        let evaluated_val = test_eval(input);
        test_integer_object(evaluated_val, 4);
    }

    #[test]
    fn test_error_handling() {
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
            Test {
                input: "foobar".to_string(),
                expected: "identifier not found: foobar".to_string(),
            },
            Test {
                input: "\"Hello\" - \"World\"".to_string(),
                expected: "unknown operator StringLiteral { value: \"Hello\" } - StringLiteral { value: \"World\" }".to_string(),
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

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World\"".to_string();

        let evaluated_val = test_eval(input);
        let string_object = match evaluated_val {
            Object::STRING(o) => o,
            other => panic!("Expected String object. Got: {:?}", other),
        };

        if string_object.value != "Hello World".to_string() {
            panic!("StringObject has wrong value. Got: {}", string_object.value);
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = "\"Hello\" + \" \" + \"World\"".to_string();

        let evaluated_val = test_eval(input);
        let string_object = match evaluated_val {
            Object::STRING(o) => o,
            other => panic!("Expected String object. Got: {:?}", other),
        };

        if string_object.value != "Hello World".to_string() {
            panic!("StringObject has wrong value. Got: {}", string_object.value);
        }
    }

    #[test]
    fn test_builtin_fn_len() {
        struct Test {
            input: String,
            expected: String,
        }

        let tests = [
            Test {
                input: "len(\"\")".to_string(),
                expected: 0.to_string(),
            },
            Test {
                input: "len(\"four\")".to_string(),
                expected: 4.to_string(),
            },
            Test {
                input: "len(\"hello world\")".to_string(),
                expected: 11.to_string(),
            },
            Test {
                input: "len(1)".to_string(),
                expected: "argument to `len` not supported, got INTEGER".to_string(),
            },
            Test {
                input: "len(\"one\", \"two\")".to_string(),
                expected: "wrong number of arguments. got=2, want=1".to_string(),
            },
            Test {
                input: "first()".to_string(),
                expected: "wrong number of arguments. got=0, want=1".to_string(),
            },
            Test {
                input: "first([], [])".to_string(),
                expected: "wrong number of arguments. got=2, want=1".to_string(),
            },
            Test {
                input: "first([])".to_string(),
                expected: "null".to_string(),
            },
            Test {
                input: "first([1, 2, 3])".to_string(),
                expected: "1".to_string(),
            },
            Test {
                input: "last()".to_string(),
                expected: "wrong number of arguments. got=0, want=1".to_string(),
            },
            Test {
                input: "last([], [])".to_string(),
                expected: "wrong number of arguments. got=2, want=1".to_string(),
            },
            Test {
                input: "last([])".to_string(),
                expected: "null".to_string(),
            },
            Test {
                input: "last([1, 2, 3])".to_string(),
                expected: "3".to_string(),
            },
            Test {
                input: "rest()".to_string(),
                expected: "wrong number of arguments. got=0, want=1".to_string(),
            },
            Test {
                input: "rest([], [])".to_string(),
                expected: "wrong number of arguments. got=2, want=1".to_string(),
            },
            Test {
                input: "rest([])".to_string(),
                expected: "null".to_string(),
            },
            Test {
                input: "rest([1, 2, 3])".to_string(),
                expected: "[2, 3]".to_string(),
            },
            Test {
                input: "push()".to_string(),
                expected: "wrong number of arguments. got=0, want=2".to_string(),
            },
            Test {
                input: "push([])".to_string(),
                expected: "wrong number of arguments. got=1, want=2".to_string(),
            },
            Test {
                input: "push([], 1)".to_string(),
                expected: "[1]".to_string(),
            },
        ];
        for test in tests {
            let evaluated_val = test_eval(test.input.clone());

            match evaluated_val {
                Object::INTEGER(num) => {
                    if num.value != test.expected.parse().unwrap() {
                        panic!(
                            "{}'s length is {}. Got:  {}",
                            test.input, test.expected, num.value
                        );
                    }
                }
                Object::ERROR(err) => {
                    if err.msg != test.expected {
                        panic!(
                            "Wrong err msg. Expected: {}, got: {}",
                            test.expected, err.msg
                        )
                    }
                }
                Object::NULL(_) => {
                    if test.expected != "null".to_string() {
                        panic!("Expected: {}, got: Null", test.expected)
                    }
                }
                Object::ARRAY(arr) => {
                    if arr.inspect() != test.expected {
                        panic!("Expected: {}, got: {}", test.expected, arr.inspect())
                    }
                }
                other => panic!("Expected Integer/Error object. Got: {:?}", other),
            };
        }
    }

    #[test]
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]".to_string();

        let evaluated_val = test_eval(input);

        let arr = match evaluated_val {
            Object::ARRAY(o) => o,
            other => panic!("Object is not of type Array. Got:  {:?}", other),
        };

        if arr.elements.len() != 3 {
            panic!("Array has wrong no. of elements");
        }

        test_integer_object(arr.elements[0].clone(), 1);
        test_integer_object(arr.elements[1].clone(), 4);
        test_integer_object(arr.elements[2].clone(), 6);
    }

    #[test]
    fn test_array_index_expressions() {
        struct Test {
            input: String,
            expected: Option<i64>,
        }

        let tests = [
            Test {
                input: "[1, 2, 3][0]".to_string(),
                expected: Some(1),
            },
            Test {
                input: "[1, 2, 3][1]".to_string(),
                expected: Some(2),
            },
            Test {
                input: "[1, 2, 3][2]".to_string(),
                expected: Some(3),
            },
            Test {
                input: "let i = 0; [1][i];".to_string(),
                expected: Some(1),
            },
            Test {
                input: "[1, 2, 3][1 + 1];".to_string(),
                expected: Some(3),
            },
            Test {
                input: "let myArray = [1, 2, 3]; myArray[2];".to_string(),
                expected: Some(3),
            },
            Test {
                input: "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];".to_string(),
                expected: Some(6),
            },
            Test {
                input: "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]".to_string(),
                expected: Some(2),
            },
            Test {
                input: "[1, 2, 3][3]".to_string(),
                expected: None,
            },
            Test {
                input: "[1, 2, 3][-1]".to_string(),
                expected: None,
            },
        ];

        for test in tests {
            let evaluated_val = test_eval(test.input);

            match test.expected {
                Some(val) => test_integer_object(evaluated_val, val),
                None => test_null_object(evaluated_val),
            }
        }
    }

    #[test]
    fn test_hash_literals() {
        let input = r#"let two = "two";
                {
                    "one": 10 - 9,
                    two: 1 + 1,
                    "thr" + "ee": 6 / 2,
                    4: 4,
                    true: 5,
                    false: 6
                }"#
        .to_string();

        let evaluated_val = test_eval(input);

        let hash_object = match evaluated_val {
            Object::HashLitearl(o) => o,
            other => panic!("Expected object of type hash. Got: {:?}", other),
        };

        let mut expected = HashMap::new();
        expected.insert(
            StringLiteral {
                value: "one".to_string(),
            }
            .hash_key(),
            1,
        );
        expected.insert(
            StringLiteral {
                value: "two".to_string(),
            }
            .hash_key(),
            2,
        );
        expected.insert(
            StringLiteral {
                value: "three".to_string(),
            }
            .hash_key(),
            3,
        );
        expected.insert(Integer { value: 4 }.hash_key(), 4);
        expected.insert(Boolean { value: true }.hash_key(), 5);
        expected.insert(Boolean { value: false }.hash_key(), 6);

        if hash_object.pairs.len() != expected.len() {
            panic!(
                "Hash has wrong no. of pairs. Got: {}",
                hash_object.pairs.len()
            );
        };

        for (expected_k, expected_v) in expected {
            let pair = match hash_object.pairs.get(&expected_k) {
                Some(p) => p,
                None => panic!("No pair for given key found in hash_object"),
            };

            test_integer_object(pair.value.clone(), expected_v);
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

        eval(program)
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
