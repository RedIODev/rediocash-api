use rediocash_api::event::Event;




fn main() {
    let mut x = X {event: Event::new() };
    x.t();
}

pub struct X {
    pub event: Event<i32, i32>
}

impl X {
    fn foo(i: i32) -> i32 {
        i
    }

    fn bar(i: i32) -> i32 {
        i
    }

    pub fn t(&mut self) {
        println!("before: {:?}", self.event.listeners.len());
        self.event.insert(|i| i+1);
        self.event += Self::foo;
        self.event += Self::bar;
        println!("after add: {:?}", self.event.listeners.len());
        self.event -= Self::foo;
        println!("after rem: {:?}", self.event.listeners.len());
    }
}

