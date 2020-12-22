
fn main() {
   
   
   //you can match to a range of Chars or Ints
   let x = 'b';
    match x {
        'a'..='e' => println!("one through five"),
        _ => println!("something else"),
    }
    
    //While there is some(thing) then do something.
    let mut vector = vec![1,2,3];
    while let Some(a) = vector.pop() {
        println!("{}",a);
    }
    
    //Create your own type then while there is some(thing) then do something.
    #[derive(Debug)]
    enum CharInt {
        Int(i32),
        Char(char),
    }
    let mut vector2: Vec<CharInt> = vec![CharInt::Int(1),CharInt::Char('h'),];
    let mut vector2_iter = vector2.iter();
    println!("{:?}", vector2[0]);
    while let Some(a) = vector2_iter.next() {
        println!("{:?}", a);
    }
    
    //See if an item in the list contains l or r and if it does then figure out what its associated number is
    let vector3: Vec<&str> = vec!["l30", "r60", "l90",];
    let mut vector3_iter = vector3.iter();
    
    while let Some(a) = vector3_iter.next() {
        let mut characters = a.chars();
        println!("{:?}", characters);
        while let Some(b) = characters.next() {
            println!("{}", b);
        }
    }
    
            //another way to do the same thing above. It's basically the same thing. For loops create an iter. This is 1 line shorter and more terse.
    let vector4: Vec<&str> = vec!["l30", "r60", "l90",];
    for x in vector4 {
        let mut characters = x.chars();
         println!("{:?}", characters);
         while let Some(b) = characters.next() {
            println!("{}", b);
        }
    }
    
    
                //one step further however I want to remove the unwraps. I also need to turn the numbers inti ints so that I can actually do calculations with them.
    let vector5: Vec<&str> = vec!["l30", "r60", "l90",];
    for x in vector5 {
        let mut characters = x.chars();
         println!("{:?}", characters);
         while let Some(b) = characters.next() {
            match b {
                'l' => println!("This is an l and it means to go left by {}{} degrees", characters.next().unwrap(), characters.next().unwrap()),
                'r' => println!("This is a r and it means to go right by {}{} degrees", characters.next().unwrap(), characters.next().unwrap()),
                _ => println!("This is not a letter"),
            }
        }
    }
    
    
    
}