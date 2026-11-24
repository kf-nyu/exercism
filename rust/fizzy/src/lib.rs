// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::ops::Rem;
/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    predicate: Box<dyn Fn(T) -> bool>,
    substitution: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: ToString>(matcher: F, subs: S) -> Matcher<T> {
        Self {
            predicate: Box::new(matcher),
            substitution: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: IntoIterator<Item = T>,
        T: ToString + Copy,
    {
        iter.into_iter().map(move |i| {
            let output = self
                .0
                .iter()
                .filter(|matcher| (matcher.predicate)(i))
                .map(|matcher| matcher.substitution.as_str())
                .collect::<String>();
            if output.is_empty() {
                i.to_string()
            } else {
                output
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Rem<Output = T> + PartialEq + From<u8> + Copy>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(
            |n: T| n % T::from(3_u8) == T::from(0_u8),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |n: T| n % T::from(5_u8) == T::from(0_u8),
            "buzz",
        ))
}
