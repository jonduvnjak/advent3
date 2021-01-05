
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
    
    
    
                //Both of the below methods for l and r will get the relevant numbers. The r version is cleaner but you need to how many ints there will be. The l version uses a loop to solve this issue.
    const RADIX: u32 = 10;

    let vector6: Vec<&str> = vec!["l30", "r60", "l90",];
    for x in vector6 {
        let mut characters = x.chars();
         //println!("{:?}", characters);
         while let Some(b) = characters.next() {
            match b {
                'l' => {println!("This is an l and it means to go left by the following degrees");
                while let Some(c) = characters.next() {match c {
                    _ => println!("{}",c.to_digit(RADIX).unwrap())
                }}},
                'r' => println!("This is a r and it means to go right by {}{} degrees", characters.next().unwrap().to_digit(RADIX).unwrap(), characters.next().unwrap().to_digit(RADIX).unwrap()),
                _ => println!("This is not a letter"),
            }
        }
    }
    
    
                   //cleaned up the above. However this feels a bit verbose. The next step is to join the numbers back together. So that 6 and 0 becomes 60. This is so that in the next step afterwards we can use the number to change the direction of the ship.
    let vector7: Vec<&str> = vec!["l30", "r60", "l90",];
    for x in vector7 {
        let mut characters = x.chars();
         //println!("{:?}", characters);
         match characters.next() {
            Some('l') => 
                {println!("This is an l and it means to go left by the following degrees");
                while let Some(c) = characters.next() {
                match c {
                    _ => println!("{}", c.to_digit(RADIX).unwrap()),
                  }
                  }
                  },
            Some('r') =>
                {println!("This is an r and it means to go right by the following degrees");
                while let Some(c) = characters.next() {
                match c {
                    _ => println!("{}", c.to_digit(RADIX).unwrap()),
                  }
                  }
                  },
            _ => println!("nothing"),
            }
            }
            
                               //Made the above much more terse and also made it so that the resulting "degree" amount is given as a u32. I also learned how powerful collect is and that parse can be used to get all the ints. One issue though is you need to be sure that there are no chars, especially when only using unwrap. I also learned about the turbofish - ::<>. This helps the inference algorithm understand specifically which type you're trying to parse into.
    let vector8: Vec<&str> = vec!["l30", "r60", "l90",];
    for x in vector8 {
        let mut characters = x.chars();
         //println!("{:?}", characters);
         match characters.next() {
            Some('l') => 
                { println!("This is an l and it means to go left by the following degrees! {:?}", characters.collect::<String>().parse::<u32>().unwrap());
                  },
            Some('r') =>
                { println!("This is an r and it means to go right by the following degrees {:?}", characters.collect::<String>().parse::<u32>().unwrap());
                  },
            _ => println!("nothing"),
            }
            }
}