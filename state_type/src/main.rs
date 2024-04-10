const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// States for the Progress
struct Unbounded;
struct Bounded {
    bound : usize,
    delims : (char, char),
}

trait ProgressDisplay : Sized {
    fn display<Iter>(&self, progress: Progress<Iter, Self>);
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: Progress<Iter, Self>) {
        println!("bounded");
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: Progress<Iter, Self>) {
        println!("unbounded");
    }
}

struct Progress<Iter, Bound> {
    iter : Iter,
    i: usize,
    bound : Bound,
}

impl<Iter> Progress<Iter, Unbounded> {
    fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, bound: Unbounded }
    }
}

// Implement some logic only when an Item is of a particular type
// (or implemts a particular Trait)
impl<Iter> Progress<Iter, Unbounded>
where Iter : ExactSizeIterator {
    pub fn with_bound(mut self) -> Progress<Iter, Bounded> {
        let bound = Bounded { bound : self.iter.len(), delims : ('[', ']') };

        Progress { iter: self.iter, i : self.i, bound }
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where Iter : Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Decorator Patter with Rust
/// Extending a type with my own method(s)
trait ProgressBarExt : Sized {
    fn progress<Bound>(self) -> Progress<Self, Bound>;
}

/// The implementation of the Decorator Pattern
/// with the constraint on the generic type T for all Iterators
impl<T> ProgressBarExt for T
where T : Iterator {
    fn progress<Bound>(self) -> Progress<T, Bound> {
        Progress::new(self)
    }
}

fn main() {
    // let vi = v.iter().progress();
    // unbounded...
    // for n in (0..).progress().with_bound() {  // ERROR _ Unbounded
    //     println!("{n}) Hello, world!");
    // }

    //bounded...
    let v = vec![1,2,3];
    for n in v.iter().progress().with_bound() {
        println!("{n}) Hello, world!");
    }
}
