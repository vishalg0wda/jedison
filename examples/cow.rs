#![allow(dead_code)]
use std::borrow::Cow;

/**
illustrating usage of std::borrow::Cow
signature:

enum Cow<'a, T>
where T: ToOwned + ?Sized + 'a
 {
  Owned(<T as ToOwned>::Owned),
  Borrowed(&'a T)
}

?Sized and 'a specify that the type may/may not be Sized, and lives at least as long as 'a.

usecase: used to implement allocation-light algorithms. a Cow provides "optional ownership".
         though, it is limited to types that implemented ToOwned, namely:
         - T: Clone -> T
         - [T] -> Vec<_>
         - str -> String
         - ... and a few others.


example usecase: Given a slice of PartialEqs and a value that can be matched against it, implement a function `filter`
                 that returns an owned vector if there is a match, or return the original slice reference.
**/

struct Matcher<T> {
    items: Vec<T>,
}

impl<T> Matcher<T>
where
    T: PartialEq + Copy,
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn filter_out(&self, against: T) -> Cow<'_, [T]> {
        let mut hit = false;
        for ele in self.items.iter() {
            if *ele == against {
                hit = true;
                break;
            }
        }
        if hit {
            let mut items = vec![];
            for ele in self.items.iter() {
                if *ele == against {
                    continue;
                }
                items.push(*ele);
            }
            return Cow::Owned(items);
        } else {
            return Cow::Borrowed(&self.items);
        }
    }
}

#[test]
fn it_works() {
    let m = Matcher {
        items: (0..100).collect(),
    };

    let cow = m.filter_out(42);
    assert!(matches!(cow, Cow::Owned(_)));
    let cow = m.filter_out(101);
    assert!(matches!(cow, Cow::Borrowed(_)));
}

fn main() {}
