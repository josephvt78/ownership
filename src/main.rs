fn main() {
    // A variable and the scope in which it is valid
    {                       // _s is not valid here, it's not yet declared
        let _s = "hello";   //_s is valid from this point forward

        //do stuff with _s
    }                       // this scope is now over, and _s is no longer valid

    // use of String as an example to study ownership
    {
        let mut s = String::from("hello"); // s is valid from this point forward
        s.push_str(", world!");                // do stuff with s
        println!("{}", s);
    }                                                // the scope is now over, and s is no
                                                     // longer valid

    //Variables and Data interacting with Move
    let x = 5;
    let _y = x;
    println!("x = {}, _y = {}", x, _y);             // Works since integer is annotated with
                                                    // Copy trait

    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{s1}"); //can't reference s1 anymore as it is 'moved'

    //Variables and Data interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    
    //Ownership and Functions
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("s1 = {}", s1); //s1 is already moved. can't borrow here

    let x = 5;
    makes_copy(x);
    println!("x = {}", x);
}

fn makes_copy(p0: i32) {
    println!("{p0} is the copy of {p0}");
}

fn takes_ownership(p0: String) {
    println!("{}", p0);
}
