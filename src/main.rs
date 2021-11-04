struct Opt<'f> {
    filter: &'f mut dyn FnMut(i32) -> bool,
}

impl<'f> Opt<'f> {
    fn new() -> Opt<'static> {
        fn always_true(_i: i32) -> bool {
            true
        }
        Opt {
            filter: &mut always_true,
        }
    }

    fn run(self) {
        for i in 0..10 {
            if (self.filter)(i) {
                println!("{}", i);
            }
        }
    }
}

fn main() {
    Opt::new().run();
}
