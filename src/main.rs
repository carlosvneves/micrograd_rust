use core::fmt;
use std::ops::{Add, Mul};

#[warn(dead_code)]
#[derive(Clone, Debug)]
struct Value {
    data: f32,
    op: Option<String>,
    label: Option<String>,
    children: Option<Vec<Value>>,
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

fn main() {
    let a = Value {
        data: 4.0,
        label: Some( "a".to_string()),
        op: None,
        children: None,
    };

    let b = Value {
        data: 2.0,
        label: Some("b".to_string()),
        op: None,
        children: None,
    };

    let mut d = a * b;
    d.label = Some("d".to_string());

    println!("{}",d);
}
