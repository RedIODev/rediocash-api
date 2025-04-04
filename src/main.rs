
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
        println!("before: {:?}", self.event.len());
        
        let cl = |i| i+1; 
        self.event += cl.clone();
        self.event += Self::foo;
        self.event += Self::bar;
        println!("after add: {:?}", self.event.len());
        self.event -= &Self::foo;
        self.event -= &cl;
        println!("after rem: {:?}", self.event.len());
    }
}

