fn main() {
    
    let animals = vec!["Rabbit","Dog","Cat"];
    for (index,a) in animals.iter().enumerate()
    {
        println!("the index value is {} and the animals name is {}", index,a);
    }
    
}
