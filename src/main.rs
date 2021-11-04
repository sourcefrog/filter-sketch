/*! An experiment in accumulating callback closures into a builder pattern.
 */

struct Opt<'f> {
    filter: Option<&'f mut dyn FnMut(i32) -> bool>,
    printer: Option<&'f dyn Fn(i32)>,
}

impl<'f> Opt<'f> {
    fn new() -> Opt<'f> {
        Opt {
            filter: None,
            printer: Some(&|i| println!("{}", i)),
        }
    }

    fn filter(self, filter: &'f mut dyn FnMut(i32) -> bool) -> Opt<'f> {
        Opt {
            filter: Some(filter),
            printer: self.printer,
        }
    }

    fn printer(self, printer: &'f dyn Fn(i32)) -> Opt<'f> {
        Opt {
            filter: self.filter,
            printer: Some(printer),
        }
    }

    fn run(&mut self) {
        for i in 0..10 {
            if let Some(ref mut filter) = self.filter {
                if filter(i) {
                    if let Some(printer) = self.printer {
                        printer(i)
                    }
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
    Opt::new().filter(&mut is_odd).run();

    println!("stateless closure");
    Opt::new().filter(&mut |i| i % 3 == 0).run();

    println!("read local variable");
    let primes = vec![2, 3, 5, 7];
    Opt::new().filter(&mut |i| primes.contains(&i)).run();

    println!("mutate local variable");
    let mut seen: Vec<i32> = Vec::new();
    Opt::new()
        .filter(&mut |i| {
            seen.push(i);
            true
        })
        .run();
    assert_eq!(seen.len(), 10);

    println!("with two closures");
    Opt::new()
        .filter(&mut |i| i & 2 == 0)
        .printer(&|i| println!("{:x}", i))
        .run();
}
