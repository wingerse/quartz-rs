use std::any::Any;
use std::collections::VecDeque;
use std::iter::Iterator;
use std::time::Duration;

pub fn duration_total_ms(dur: Duration) -> f64 {
    let total_secs = dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000.0;
    total_secs * 1000.0
}

pub fn iter_foreach_every<I, F, P>(i: I, p: P, mut f: F)
where
    I: Iterator,
    F: FnMut(&mut VecDeque<I::Item>),
    P: Fn(usize) -> bool,
{
    let mut q = VecDeque::new();
    for (i, v) in i.enumerate() {
        q.push_back(v);
        if p(i) {
            f(&mut q);
        }
    }
    if !q.is_empty() {
        f(&mut q);
    }
}

macro_rules! impl_from_for_newtype_enum {
    ($enum_:ident::$variant:ident, $other:ty) => {
        impl ::std::convert::From<$other> for $enum_ {
            fn from(x: $other) -> Self {
                $enum_::$variant(x)
            }
        }
    };
}
