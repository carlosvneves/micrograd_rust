use core::fmt;
use std::ops::{Add, Mul};

#[warn(dead_code)]
#[derive(Clone, Debug)]
pub struct Value {
    pub data: f32,
    pub op: Option<String>,
    pub label: Option<String>,
    pub children: Option<Vec<Value>>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let op = match self.op.clone() {
            Some(op) => op,
            None => "None".to_string(),
            
        };
        
        let label = match self.label.clone() {
            Some(label) => label,
            None => "None".to_string(),
            
        };
        
        let children = match self.children.clone() {
            Some(children) => children,
            None => vec![],
            
        };
        
        let var_name = write!(
            f,
            "Value=(data={}, op={:?}, _label={:?}, _children: [{:#?}, {:#?}])",
            self.data,
            Some(op),
            Some(label),
            Some(children[0].clone()),
            Some(children[1].clone())
        );
        var_name
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self: Value, other: Value) -> Value {
        let op = "+".to_string();   
        let label = match self.label.clone() {
            Some(label) => label,
            None => "None".to_string(),
        };
        let new_value = Value {
            data: self.data + other.data,
            op: Some(op),
            label: Some(label),
            children: Some(vec![self, other]),
            
        };
        
        return new_value;
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self: Value, other: Value) -> Value {
        let op = "*".to_string();
        
        let label = match self.label.clone() {
            Some(label) => label,
            None => "None".to_string(),
        };
        
        
        Self{
            data: self.data * other.data,
            op: Some(op),
            label: Some(label),
            children: Some(vec![self, other]),
        }
    }
}