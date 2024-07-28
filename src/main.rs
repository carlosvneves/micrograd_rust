mod engine;

use engine::value::Value;
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
