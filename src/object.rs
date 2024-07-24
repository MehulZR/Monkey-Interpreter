use std::{fmt::Debug, ops::Deref};

pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
    RETURN,
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
        }
    }
}
