#![allow(dead_code)]
#![allow(unused_variables)]

trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }

    fn create(name : &'static str) -> Self;
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {

    fn create(name: &'static str) -> Human {
        Human {name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self){
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat {name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self){
        println!("{} says miau", self.name);
    }
}

pub fn traits() {
    let h = Human {name: "Chris"};
    h.talk();

    let c = Cat {name: "Misty"};
    c.talk();

    let h2 = Human::create("Christian");
    h2.talk();

    let h3:Human = Human::create("Christiano");
    h3.talk();

}