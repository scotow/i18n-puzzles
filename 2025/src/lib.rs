#[macro_export]
macro_rules! main {
    () => {
        #[allow(unused_import)]
        use {
            itertools::Itertools,
            jiff::{Offset, SignedDuration, Span, TimeZone, Timestamp, Zoned},
            std::cmp::Ordering,
            std::collections::{HashMap, HashSet, VecDeque, hash_map::DefaultHasher},
            std::convert::identity,
            std::fmt::{Debug, Display, Formatter},
            std::hash::{Hash, Hasher},
            std::iter::{Peekable, once, successors, zip},
            std::mem::{replace, swap, take},
            std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Range, RangeInclusive, Sub},
            std::rc::Rc,
            std::str::{FromStr, from_utf8},
        };

        fn main() {
            println!(
                "{}",
                run(include_str!(concat!("../input/", module_path!(), ".txt")).trim_end())
            );
        }
    };
}
