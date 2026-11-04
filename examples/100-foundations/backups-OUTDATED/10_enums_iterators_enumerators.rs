#[derive(Debug)]
struct Individual {
    name: String,
}

enum Room {
    Occupied(Individual),
    Vacant,
}

type Hotel = Vec<Room>;

fn main() {
    let mut hotel = Hotel::new();

    let alice = Individual {
        name: "Alice".to_string(),
    };

    let barbara = Individual {
        name: "Barbara".to_string(),
    };

    hotel.push(Room::Occupied(alice));
    hotel.push(Room::Vacant);
    hotel.push(Room::Occupied(barbara));

    for room in hotel.iter() {
        match *room {
            Room::Vacant => println!("Available!"),
            Room::Occupied(_) => println!("Taken, sorry."),
        }
    }

    for (i, house) in hotel.iter().enumerate() {
        match *house {
            Room::Vacant => println!("No. {} is available!", i),
            Room::Occupied(_) => println!("No {} is taken.", i),
        }
    }

    //step 3, introduce scoping, ref keuword
    use Room::{Occupied, Vacant};

    for (i, house) in hotel.iter().enumerate() {
        match *house {
            Vacant => println!("No. {} is available!", i),
            Occupied(ref occupant) => println!("No {} is taken by {:?}.", i, occupant.name),
        }
    }
}
