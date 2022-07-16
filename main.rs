use std::time::Duration;

fn main();
{
    println!("To the Fizz and the Buzz");
    println!("input number : ");
    let mut line = string::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println("--------------");
    let upto : i32 = line.trim().parse().unwap();
    for i in vector.into_iter()[
        thred::sleep(Duration::from_millis(1000));
        println!("{}", i);
    ]
}

fn fizzbuzz(upto : i32) -> Vec<String>{
    let mut result : Vec<String> = Vec::new();
    for i in 0 ..upto{
        result.push(match (i % 3, i % 5){
            (0.0) => "FizzBuzz".into(),
            (0,_) => "Fizz".into(),
            (_,0) => "Buzz".into(),
            (_,_) => i.to_string()
        })
    }
}
