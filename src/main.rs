// trait Filter = FnMut(i32) -> bool;

struct Opt<'f> {
    filter: Option<&'f mut dyn FnMut(i32) -> bool>,
}

impl<'f> Opt<'f> {
    fn new() -> Opt<'static> {
        Opt { filter: None }
    }

    fn filter<'g>(self, filter: &'g mut dyn FnMut(i32) -> bool) -> Opt<'g> {
        Opt {
            filter: Some(filter),
        }
    }

    fn run(&mut self) {
        for i in 0..10 {
            if let Some(ref mut filter) = self.filter {
                if filter(i) {
                    println!("{}", i);
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
}
