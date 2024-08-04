mod human;

fn main() {
    let human = human::Human::new("John".to_string(), 30);
    println!("Age: {}", human.get_age());
    println!("Name: {}", human.get_name());
    
    let child = human::Child::new("Jane".to_string(), 5);
    println!("Age: {}", child.get_age());
    println!("Name: {}", child.get_name());
}
