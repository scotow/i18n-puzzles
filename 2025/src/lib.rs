#[macro_export]
macro_rules! main {
    () => {
        #[allow(unused_import)]
        use {
            deunicode::deunicode,
            itertools::Itertools,
            jiff::{
                SignedDuration, Span, Timestamp, Zoned,
                civil::{Date, date, datetime, time},
                tz::{Offset, TimeZone},
            },
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
            unicode_normalization::{Decompositions, char::compose},
        };

        fn main() {
            println!(
                "{}",
                run(include_str!(concat!("../input/", module_path!(), ".txt")).trim_end())
            );
        }
    };
}
