use public_transport::BusNetwork;

mod public_transport {
    use std::cell::RefCell;
    use std::rc::Rc;

    type BusStopRef = Rc<RefCell<BusStop>>;

    /// A bus stop in a public transport network.
    #[derive(Debug)]
    pub struct BusStop {
        /// The name of the bus stop.
        name: String,

        /// The destinations reachable from this bus stop.
        destinations: Vec<BusStopRef>,
    }

    impl BusStop {
        /// Creates a new bus stop with the given name.
        fn new(name: &str) -> BusStopRef {
            Rc::new(RefCell::new(BusStop {
                name: name.to_string(),
                destinations: Default::default(),
            }))
        }

        fn add_edge(&mut self, target: BusStopRef) {
            self.destinations.push(target);
        }

        pub fn name(&self) -> &str {
            &self.name
        }
    }

    /// A public transport network.
    #[derive(Debug)]
    pub struct BusNetwork {
        stops: Vec<BusStopRef>,
    }

    impl BusNetwork {
        /// Returns the bus stop with the given name, if it exists.
        fn get_stop(&self, name: &str) -> Option<BusStopRef> {
            self.stops
                .iter()
                .find(|stop| stop.borrow().name == name)
                // Clone the reference to the stop if it exists.
                .map(Clone::clone)
        }

        /// Creates a new, empty public transport network.
        pub fn new() -> BusNetwork {
            BusNetwork {
                stops: Default::default(),
            }
        }

        /// Adds a new bus stop to the network, or returns the existing one with the given name.
        pub fn add_or_get_stop(&mut self, name: &str) -> BusStopRef {
            if let Some(stop) = self.get_stop(name) {
                stop
            } else {
                let new_stop = BusStop::new(name);
                // Add a clone of the reference to the new stop to the network.
                self.stops.push(Clone::clone(&new_stop));
                new_stop
            }
        }

        /// Adds a route between two bus stops. Stops are created if they don't exist yet.
        pub fn add_route(&mut self, source_name: &str, target_name: &str) {
            let source_stop = self.add_or_get_stop(source_name);
            let target_stop = self.add_or_get_stop(target_name);
            source_stop.borrow_mut().add_edge(target_stop);
        }

        /// Returns the destinations reachable from the given bus stop.
        /// If the stop doesn't exist, an empty vector is returned.
        pub fn destinations(&self, source_name: &str) -> Vec<BusStopRef> {
            let source_stop = self.get_stop(source_name);
            if let Some(source_stop) = source_stop {
                source_stop.borrow().destinations.clone()
            } else {
                Vec::new()
            }
        }

        /// Renames a bus stop.
        pub fn rename_stop(&mut self, old_name: &str, new_name: &str) {
            if let Some(stop) = self.get_stop(old_name) {
                // Mutable borrow the stop and change its name.
                stop.borrow_mut().name = new_name.to_string();
            }
        }

        /// Helper method to dump the network to stdout. 
        /// Includes the number of references to each stop.
        pub fn dump(&self, title: &str) {
            println!("=== {title}");
            for stop in &self.stops {
                println!("Stop {} ({})", stop.borrow().name, Rc::strong_count(stop));
            }
        }
    }
}

fn main() {
    let mut network = BusNetwork::new();

    // Add three stops to the network.
    network.add_or_get_stop("Vienna");
    network.add_or_get_stop("Grz");
    network.add_or_get_stop("Linz");
    network.dump("Ref count of all stops is 1 because each stop is only referenced once by the network");

    network.add_route("Vienna", "Grz");
    network.dump("Ref count to Graz should be 2 because it is referenced by the network and by Vienna");
    let d = network.destinations("Vienna");
    network.dump("ref count of Graz should be 3 because it is referenced by the network, by Vienna and by the destinations vector");

    // Rename the stop "Grz" to "Graz". Influences all references to the stop.
    network.rename_stop("Grz", "Graz");
    println!(
        "First destination from Vienna: {:?}",
        d.first().unwrap().borrow().name()
    );
    drop(d);

    network.add_route("Linz", "Graz");
    network.dump("Ref count to Graz should be 3 because it is referenced by the network, by Vienna, and by Linz");

    for stop in network.destinations("Vienna") {
        println!("From Vienna you can go to {}", stop.borrow().name());
    }
}
