use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::Deref, rc::Rc};

use lazy_static::lazy_static;

use crate::ast::{BlockStatement, Identifier, Node};

pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
    RETURN,
    ERROR,
    FUNCTION,
    STRING,
    BUILTINFUNC,
}

pub trait ObjectTrait {
    fn r#type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Object {
    INTEGER(Integer),
    BOOLEAN(Boolean),
    NULL(Null),
    RETURN(Return),
    ERROR(Error),
    FN(Function),
    STRING(StringLiteral),
    BUILTINFUNC(BuiltInFunc),
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64,
}

impl ObjectTrait for Integer {
    fn r#type(&self) -> ObjectType {
        ObjectType::INTEGER
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl ObjectTrait for Boolean {
    fn r#type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

impl ObjectTrait for StringLiteral {
    fn r#type(&self) -> ObjectType {
        ObjectType::STRING
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }
}
#[derive(Debug, Clone)]
pub struct Null {}

impl ObjectTrait for Null {
    fn r#type(&self) -> ObjectType {
        ObjectType::NULL
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub value: Box<Object>,
}

impl ObjectTrait for Return {
    fn r#type(&self) -> ObjectType {
        ObjectType::RETURN
    }

    fn inspect(&self) -> String {
        match self.value.deref() {
            Object::INTEGER(o) => o.inspect(),
            Object::BOOLEAN(o) => o.inspect(),
            Object::NULL(o) => o.inspect(),
            Object::RETURN(o) => o.inspect(),
            Object::ERROR(o) => o.inspect(),
            Object::FN(o) => o.inspect(),
            Object::STRING(o) => o.inspect(),
            Object::BUILTINFUNC(o) => o.inspect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
}

impl ObjectTrait for Error {
    fn r#type(&self) -> ObjectType {
        ObjectType::ERROR
    }

    fn inspect(&self) -> String {
        format!("Error: {}", self.msg.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

impl ObjectTrait for Function {
    fn r#type(&self) -> ObjectType {
        ObjectType::FUNCTION
    }

    fn inspect(&self) -> String {
        let mut str = String::new();
        let params = self
            .params
            .iter()
            .map(|param| param.string())
            .collect::<Vec<String>>()
            .join(", ");

        str.push_str("fn(");
        str.push_str(&params);
        str.push_str("){\n");
        str.push_str(&self.body.string());
        str.push_str("\n}");

        str
    }
}

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &String) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer_env) => outer_env.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, obj: Object) {
        self.store.insert(name, obj.clone());
    }
}

pub fn enclosed_environment(outer_env: &Rc<RefCell<Environment>>) -> Environment {
    Environment {
        store: HashMap::new(),
        outer: Some(Rc::clone(outer_env)),
    }
}

#[derive(Debug, Clone)]
pub struct BuiltInFunc {
    pub func: fn(arg: Vec<Object>) -> Object,
}
impl ObjectTrait for BuiltInFunc {
    fn r#type(&self) -> ObjectType {
        ObjectType::BUILTINFUNC
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

lazy_static! {
    pub static ref BUILTINS: HashMap<&'static str, BuiltInFunc> = {
        let mut builtins = HashMap::new();
        builtins.insert("len", BuiltInFunc { func: monkey_len });
        builtins
    };
}

fn monkey_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(Error {
            msg: format!("wrong number of arguments. got={}, want={}", args.len(), 1),
        });
    }

    match &args[0] {
        Object::STRING(str) => Object::INTEGER(Integer {
            value: str.value.len() as i64,
        }),
        Object::INTEGER(_) => Object::ERROR(Error {
            msg: "argument to `len` not supported, got INTEGER".to_string(),
        }),
        other => panic!("expected monkey_len args[0] to be string. Got: {:?}", other),
    }
}
