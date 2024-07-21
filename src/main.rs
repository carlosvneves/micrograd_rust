use core::fmt;
use std::ops::{Add, Mul};

#[warn(dead_code)]

#[derive(Clone)]
struct Value{
    data: f32,
    op: String,
    label: String,
    
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value=(data={})", self.data)
        
    } 

}

impl Add for Value{
    type Output = Self;
    fn add(self:Value, other:Value) -> Value{
        Value{
            data: self.data + other.data,
            op: "+".to_string(),
            label: self.label
        }
    }
}

impl Mul for Value{
    type Output = Self;
    fn mul(self:Value, other:Value) -> Value{
        Value{
            data: self.data * other.data,
            op: "*".to_string(),
            label: self.label
        }
    }
}

fn main() {
    let  a = Value{
        data: 1.0,
        label: "a".to_string(),
        op: "".to_string(),
    };

    let  b = Value{
        data: 2.0,
        label: "b".to_string(),
        op: "".to_string(),
    };

    println!("{}", a);
    println!("{}", a.label);

    let mut c = a.clone() + b.clone();
    c.label = "c".to_string();
    c.op = "+".to_string();
    println!("{},{},{}", c, c.label, c.op);
    let mut d = a * b;
    d.label = "d".to_string();
    d.op = "*".to_string();
    println!("{},{},{}", d, d.label, d.op);
}
