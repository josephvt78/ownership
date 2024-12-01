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
}
