#[macro_export]
macro_rules! main {
    () => {
        #[allow(unused_import)]
        use {
            std::cmp::Ordering,
            std::collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
            std::convert::identity,
            std::fmt::{Debug, Display, Formatter},
            std::hash::{Hash, Hasher},
            std::iter::{once, successors, zip, Peekable},
            std::mem::{replace, take, swap},
            std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Range, RangeInclusive, Sub},
            std::rc::Rc,
            std::str::{from_utf8, FromStr},
        };

        fn main() {
            println!("{}", run(include_str!(concat!("../input/", module_path!(), ".txt")).trim_end()));
        }
    };
}