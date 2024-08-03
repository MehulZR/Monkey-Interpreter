use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

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
    ARRAY,
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
    ARRAY(Array),
}

impl ObjectTrait for Object {
    fn r#type(&self) -> ObjectType {
        match self {
            Self::INTEGER(o) => o.r#type(),
            Self::BOOLEAN(o) => o.r#type(),
            Self::NULL(o) => o.r#type(),
            Self::RETURN(o) => o.r#type(),
            Self::ERROR(o) => o.r#type(),
            Self::FN(o) => o.r#type(),
            Self::STRING(o) => o.r#type(),
            Self::BUILTINFUNC(o) => o.r#type(),
            Self::ARRAY(o) => o.r#type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            Self::INTEGER(o) => o.inspect(),
            Self::BOOLEAN(o) => o.inspect(),
            Self::NULL(o) => o.inspect(),
            Self::RETURN(o) => o.inspect(),
            Self::ERROR(o) => o.inspect(),
            Self::FN(o) => o.inspect(),
            Self::STRING(o) => o.inspect(),
            Self::BUILTINFUNC(o) => o.inspect(),
            Self::ARRAY(o) => o.inspect(),
        }
    }
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
pub struct Array {
    pub elements: Vec<Object>,
}

impl ObjectTrait for Array {
    fn r#type(&self) -> ObjectType {
        ObjectType::ARRAY
    }

    fn inspect(&self) -> String {
        let mut str = String::new();

        let items = self
            .elements
            .iter()
            .map(|param| param.inspect())
            .collect::<Vec<String>>()
            .join(", ");

        str.push_str("[");
        str.push_str(&items);
        str.push_str("]");

        str
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
        self.value.inspect()
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
        builtins.insert("first", BuiltInFunc { func: monkey_first });
        builtins.insert("last", BuiltInFunc { func: monkey_last });
        builtins.insert("rest", BuiltInFunc { func: monkey_rest });
        builtins.insert("push", BuiltInFunc { func: monkey_push });
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
        Object::ARRAY(arr) => Object::INTEGER(Integer {
            value: arr.elements.len() as i64,
        }),
        other => panic!(
            "expected `monkey_len` args[0] to be string. Got: {:?}",
            other
        ),
    }
}

fn monkey_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(Error {
            msg: format!("wrong number of arguments. got={}, want={}", args.len(), 1),
        });
    }

    match &args[0] {
        Object::ARRAY(arr) => {
            if arr.elements.len() > 0 {
                return arr.elements[0].clone();
            }
            Object::NULL(Null {})
        }
        other => panic!("expected `first` args[0] to be arr. Got: {:?}", other),
    }
}

fn monkey_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(Error {
            msg: format!("wrong number of arguments. got={}, want={}", args.len(), 1),
        });
    }

    match &args[0] {
        Object::ARRAY(arr) => {
            if arr.elements.len() > 0 {
                return arr.elements[arr.elements.len() - 1].clone();
            }
            Object::NULL(Null {})
        }
        other => panic!("expected `first` args[0] to be arr. Got: {:?}", other),
    }
}

fn monkey_rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ERROR(Error {
            msg: format!("wrong number of arguments. got={}, want={}", args.len(), 1),
        });
    }

    match &args[0] {
        Object::ARRAY(arr) => {
            if arr.elements.len() > 0 {
                return Object::ARRAY(Array {
                    elements: arr.elements[1..=arr.elements.len() - 1].to_vec(),
                });
            }
            Object::NULL(Null {})
        }
        other => panic!("expected `first` args[0] to be arr. Got: {:?}", other),
    }
}

fn monkey_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::ERROR(Error {
            msg: format!("wrong number of arguments. got={}, want={}", args.len(), 2),
        });
    }

    match &args[0] {
        Object::ARRAY(arr) => {
            let mut new_arr = arr.elements.clone();
            new_arr.push(args[1].clone());
            Object::ARRAY(Array { elements: new_arr })
        }
        other => panic!("expected `first` args[0] to be arr. Got: {:?}", other),
    }
}
