struct Person{
    name:String,
    age:i32,
}

fn main() {
    let p = Person{
        name: String::from("Alice"),
        age:30,
    };
    println!("{} is {} years old",p.name,p.age);

    let mut numbers = [1,2,3,4,5,6,8,7,9,10];
    for number in numbers{
        if number%2==0{
            println!("{}",number);
        }
    }
    let mut number =[1,2,3,4,5];
    println!("{}", addi(&number));
    
    let num :Vec<i32> = vec![1,2,3,4,5,6];
    println!("{}", vect(&num)); 
let num2 : Vec<i32> = vec![10,20,30,40,50,60,70,80];
println!("{}", maxVect(&num2));
}



fn vect(v:&[i32])->i32{
    let mut sum = 0;
    for &i in v{
       sum+=i;
    }
    sum
}
fn maxVect(v:&[i32])->i32{
    let mut max = 0;
    for &i in v{
        if i>max{
            max=i;
        }
    }
    max
}



fn addi(x:&[i32])->i32{
let mut sum = 0;
    for i in x{
      sum += i; 
    }
sum
    
}

