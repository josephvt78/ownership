fn main() {
    // A variable and the scope in which it is valid
    {                       // _s is not valid here, it's not yet declared
        let _s = "hello";   //_s is valid from this point forward

        //do stuff with _s
    }                       // this scope is now over, and _s is no longer valid
}
