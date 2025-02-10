// Loop
fn main(){
    let mut count = 0;
    'count : loop{
        println!("Count {count}");
        if count == 5{
            break 'count;
        } else {
            count += 1;
        }
    }
}

fn while() {
    let mut count = 0;
    while count < 5{
        println!("Hi {count}");
        count += 1;
    }
}