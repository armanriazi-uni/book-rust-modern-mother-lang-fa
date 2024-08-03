macro_rules! MyString {
    ($x:expr) => ( // <1>
        String::from(stringify!($x)) // <2>
    )
}
//let s = MyString!(hello there);
