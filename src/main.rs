/*! An experiment in accumulating callback closures into a builder pattern.
 *
 * If the filters are passed in as `&mut` or `&` then rustc understands how they can constrain
 * the lifetime of the build [Opt] and so they can reference and mutate local variables.
 *
 * Alternatively they can be passed by value but also with a lifetime constraint.
 * The functions that receive them by value are parameterized by the incoming
 * Fn type, but they can box it (to cope with whatever size the closure is) and
 * include a dyn trait pointer. This seems pretty nice.
 */

use std::fmt::Write;

struct Opt<'f> {
    filter: Option<Box<dyn FnMut(i32) -> bool + 'f>>,
    printer: Box<dyn FnMut(i32) + 'f>,
}

impl<'f> Opt<'f> {
    fn new() -> Opt<'f> {
        Opt {
            filter: None,
            printer: Box::new(|i| println!("{}", i)),
        }
    }

    fn filter<F>(self, filter: F) -> Opt<'f>
    where
        F: FnMut(i32) -> bool + 'f,
    {
        Opt {
            filter: Some(Box::new(filter)),
            printer: self.printer,
        }
    }

    fn printer<P>(self, printer: P) -> Opt<'f>
    where
        P: FnMut(i32) + 'f,
    {
        Opt {
            filter: self.filter,
            printer: Box::new(printer),
        }
    }

    fn run(&mut self) {
        for i in 0..10 {
            if let Some(ref mut filter) = self.filter {
                if filter(i) {
                    (self.printer)(i);
                }
            }
        }
    }
}

fn main() {
    println!("no filter");
    Opt::new().run();

    println!("inner fn filter");
    fn is_odd(i: i32) -> bool {
        i % 2 == 1
    }
    // Requirer a "&mut" which is not ideal, but it works.
    Opt::new().filter(is_odd).run();

    println!("stateless closure");
    Opt::new().filter(|i| i % 3 == 0).run();

    println!("read local variable");
    let primes = vec![2, 3, 5, 7];
    Opt::new().filter(|i| primes.contains(&i)).run();

    println!("mutate local variable");
    let mut seen: Vec<i32> = Vec::new();
    Opt::new()
        .filter(|i| {
            seen.push(i);
            true
        })
        .run();
    assert_eq!(seen.len(), 10);

    println!("with two closures");
    Opt::new()
        .filter(|i| i & 2 == 0)
        .printer(|i| println!("{:x}", i))
        .run();

    println!("accumulate results");
    let mut output = String::new();
    Opt::new()
        .filter(|i| i & 2 == 0)
        .printer(|i| write!(&mut output, "{:#x}, ", i).unwrap())
        .run();
    assert_eq!(output, "0x0, 0x1, 0x4, 0x5, 0x8, 0x9, ");
}
