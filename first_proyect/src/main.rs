fn main() {  
    let age: u8=17;
    let name: &str="Simón";
    println!("Hi my name is {} and i have {} years old",name,age);

    println!("please enter your name => ");
    let mut user :String=String::new();
    std::io::stdin().read_line(&mut user).unwrap();

    println!("Hola, {}",user);

    println!("{} enter your age",user);
    let mut age:String=String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age_int:u8=age.trim().parse().unwrap();

    if age_int>=18{
        println!("wellcome {} of {} years old",user,age_int)
    }
    else{
        println!("sorry you are a minor")
    }
}
