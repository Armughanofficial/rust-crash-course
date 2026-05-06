pub fn min(x: i32, y: i32) -> i32 {
    
    if x < y {
        println!("x is less than y");
        x
    }
    else if x > y {
        println!("x is greater than y");
        y
    }
    else {
        println!("x is equal to y");
        x
    }
}

pub fn max(x: i32, y: i32) -> i32 {
    
   
    if x < y {
        println!("x is less than y");
        y
    }
    else if x > y {
        println!("x is greater than y");
        x
    }
    else {
        println!("x is equal to y");
        x
    }
}

pub fn sign(x: i32) -> i32 {
    
    if x < 0 {    
        -1
    }
    else {
       1
    }

}

