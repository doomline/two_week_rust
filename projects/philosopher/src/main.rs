// this program is to test concurrency
/*In ancient times, a wealthy philanthropist endowed a College to accommodate five eminent philosophers. Each philosopher had a room in which they could engage in their professional activity of thinking; there was also a common dining room, furnished with a circular table, surrounded by five chairs, each labelled by the name of the philosopher who was to sit in it. They sat anticlockwise around the table. To the left of each philosopher there was laid a golden fork, and in the centre stood a large bowl of spaghetti, which was constantly replenished. A philosopher was expected to spend most of their time thinking; but when they felt hungry, they went to the dining room, sat down in their own chair, picked up their own fork on their left, and plunged it into the spaghetti. But such is the tangled nature of spaghetti that a second fork is required to carry it to the mouth. The philosopher therefore had also to pick up the fork on their right.
 When they were finished they would put down both their forks, get up from their chair, and continue thinking. Of course, a fork can be used by only one philosopher at a time. If the other philosopher wants it, they just have to wait until the fork is available again.
 */

// First create a struct called Philospher

struct Philosopher {
    name: String
}

// Next we create the implementation of the philospher
// whenever we implement philosopher we need to create a new one
// we use the name as the argument and we will reference the string to the name
// returning a philopsher with a name that will will have a value of Name but as a string

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
// added a function called eat because this is an action that the philosophers will do
    fn eat(&self) {
        println!("{} is done eating.", self.name);
}

}

fn main() {
    // created a Vector(growable array) so that we can add and take out philosophers instead of having them be their own individual type
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Giles Deleuze"),
        Philosopher::new("Mike Tyson"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michael Foucault"),
    ];
// loop through each philosopher and have them eat
    for p in &philosophers {
        p.eat();
    }
}
