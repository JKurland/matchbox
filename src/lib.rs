#[macro_export]
macro_rules! matchbox {
    // else and don't use the box
    ($b:expr, else => $e:expr$(,)?) => {{
        let _ = $b;
        $e
    }};
    // else and use the box
    ($b:expr, else:$p:pat => $e:expr$(,)?) => {{
        let $p = $b;
        $e
    }};
    // destructure a struct without repeating the type name
    ($b:expr, $t:path|{$($p:tt)+} $($rest:tt)+) => {{
        matchbox!($b, $t: $t{$($p)+} $($rest)*)
    }};
    // destructure a tuple struct without repeating the type name
    ($b:expr, $t:path|($($p:tt)+) $($rest:tt)+) => {{
        matchbox!($b, $t: $t($($p)+) $($rest)*)
    }};
    // general destructure
    ($b:expr, $t:ty:$p:pat => $e:expr, $($rest:tt)+) => {{
        match $b.downcast::<$t>() {
            Ok(_inner_box) => {
                let $p = *_inner_box;
                $e
            }
            Err(_inner_box) => {
                matchbox!{_inner_box, $($rest)*}
            }
        }
    }};
    // match without using the value
    ($b:expr, $t:ty => $e:expr, $($rest:tt)+) => {{
        match $b.downcast::<$t>() {
            Ok(_) => {
                $e
            }
            Err(_inner_box) => {
                matchbox!{_inner_box, $($rest)*}
            }
        }
    }};

    // general destructure without comma
    ($b:expr, $t:ty:$p:pat => $e:block $($rest:tt)+) => {{
        match $b.downcast::<$t>() {
            Ok(_inner_box) => {
                let $p = *_inner_box;
                $e
            }
            Err(_inner_box) => {
                matchbox!{_inner_box, $($rest)*}
            }
        }
    }};
    // match without using the value without comma
    ($b:expr, $t:ty => $e:block $($rest:tt)+) => {{
        match $b.downcast::<$t>() {
            Ok(_) => {
                $e
            }
            Err(_inner_box) => {
                matchbox!{_inner_box, $($rest)*}
            }
        }
    }};
}


#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt::{Formatter, Display};

    #[derive(Debug)]
    struct Error1 ();

    impl Display for Error1 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error1")
        }
    }

    impl Error for Error1 {}

    #[derive(Debug)]
    struct Error2 {i: i32}

    impl Display for Error2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error1")
        }
    }

    impl Error for Error2 {}

    #[derive(Debug)]
    struct Error3 (f64);

    impl Display for Error3 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error3")
        }
    }

    impl Error for Error3 {}

    #[test]
    fn get_error_type() {
        let b: Box<dyn Error> = Box::new(Error1());
        let i = matchbox!{ b,
            Error1 => 1,
            else => 0
        };

        assert_eq!(i, 1);
    }

    #[test]
    fn get_error() {
        let b: Box<dyn Error> = Box::new(Error1());
        let s = matchbox!{ b,
            Error1: e => format!("{}", e),
            else => "".to_string()
        };

        assert_eq!(s, "Error1");
    }

    #[test]
    fn multiple_arms() {
        let b: Box<dyn Error> = Box::new(Error1());
        let s = matchbox!{ b,
            Error1: e => format!("{}", e),
            Error2: e => format!("{}", e),
            else => "".to_string()
        };

        assert_eq!(s, "Error1");
    }

    #[test]
    fn destructure() {
        let b: Box<dyn Error> = Box::new(Error2{i: 3});
        let s = matchbox!{ b,
            Error1: e => format!("{}", e),
            Error2|{i} => format!("{}", i),
            else => "".to_string()
        };

        assert_eq!(s, "3");
    }


    #[test]
    fn destructure_tuple_struct() {
        let b: Box<dyn Error> = Box::new(Error3(3.2));
        let f = matchbox!{ b,
            Error1 => 0.0,
            Error2 => 0.0,
            Error3|(f) => f,
            else => 0.0
        };

        assert_eq!(f, 3.2);
    }

    #[test]
    fn just_else() {
        let _b: Box<dyn Error> = Box::new(Error3(3.2));
        let f = matchbox!{ _b,
            else => 0.0
        };

        assert_eq!(f, 0.0);
    }

    #[test]
    fn just_else_trailing_comma() {
        let _b: Box<dyn Error> = Box::new(Error3(3.2));
        let f = matchbox!{ _b,
            else => 0.0,
        };

        assert_eq!(f, 0.0);
    }

    #[test]
    fn block_no_comma() {
        let b: Box<dyn Error> = Box::new(Error3(3.2));
        let f = matchbox!{ b,
            Error3|(f) => {
                let g = f + 2.0;
                g
            }
            Error1 => {
                let x = 2.0;
                3.2 + x
            }
            Error2|{i} => {
                let x = i as f64;
                3.2 + x
            }
            else => 0.0,
        };

        assert_eq!(f, 5.2);
    }
}
