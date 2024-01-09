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

    /// Adds a new destination reachable from this bus stop.
    fn add_edge(&mut self, target: BusStopRef) {
        self.destinations.push(target);
    }

    /// Returns the name of the bus stop.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the bus stop.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

/// A public transport network.
#[derive(Debug)]
pub struct BusNetwork {
    stops: Vec<BusStopRef>,
}

impl BusNetwork {
    /// Creates a new, empty public transport network.
    pub fn new() -> BusNetwork {
        BusNetwork {
            stops: Default::default(),
        }
    }

    /// Returns the bus stop with the given name, if it exists.
    pub fn get_stop(&self, name: &str) -> Option<BusStopRef> {
        self.stops
            .iter()
            .find(|stop| stop.borrow().name == name)
            // Clone the reference to the stop if it exists.
            .map(Clone::clone)
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
            stop.borrow_mut().set_name(new_name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bus_stop = BusStop::new("Test Stop");
        assert_eq!(bus_stop.borrow().name(), "Test Stop");
    }

    #[test]
    fn test_add_edge() {
        let bus_stop1 = BusStop::new("Stop 1");
        let bus_stop2 = BusStop::new("Stop 2");
        bus_stop1.borrow_mut().add_edge(bus_stop2.clone());
        assert_eq!(bus_stop1.borrow().destinations.len(), 1);
        assert_eq!(bus_stop1.borrow().destinations[0].borrow().name(), "Stop 2");
    }

    #[test]
    fn test_set_name() {
        let bus_stop = BusStop::new("Test Stop");
        bus_stop.borrow_mut().set_name("New Stop");
        assert_eq!(bus_stop.borrow().name(), "New Stop");
    }

    #[test]
    fn test_new_network() {
        let network = BusNetwork::new();
        assert_eq!(network.stops.len(), 0);
    }

    #[test]
    fn test_get_stop() {
        let mut network = BusNetwork::new();
        network.add_or_get_stop("Stop 1");
        assert!(network.get_stop("Stop 1").is_some());
        assert!(network.get_stop("Stop 2").is_none());
    }

    #[test]
    fn test_add_or_get_stop() {
        let mut network = BusNetwork::new();
        let new_stop = network.add_or_get_stop("Stop 1");
        let existing_stop = network.add_or_get_stop("Stop 1");

        // The network should contain one stop.
        assert_eq!(network.stops.len(), 1);

        // The two stops should be the same.
        assert_eq!(Rc::ptr_eq(&new_stop, &existing_stop), true);

        // The stop should have three references: one in the network, and two in the test.
        assert_eq!(3, Rc::strong_count(&new_stop));
    }

    #[test]
    fn test_add_route() {
        let mut network = BusNetwork::new();
        network.add_route("Stop 1", "Stop 2");
        assert_eq!(network.stops.len(), 2);
        assert_eq!(
            network
                .get_stop("Stop 1")
                .unwrap()
                .borrow()
                .destinations
                .len(),
            1
        );
        assert_eq!(
            network.get_stop("Stop 1").unwrap().borrow().destinations[0]
                .borrow()
                .name,
            "Stop 2"
        );
    }

    #[test]
    fn test_destinations() {
        let mut network = BusNetwork::new();
        network.add_route("Stop 1", "Stop 2");
        let destinations = network.destinations("Stop 1");
        assert_eq!(destinations.len(), 1);
        assert_eq!(destinations[0].borrow().name, "Stop 2");
    }

    #[test]
    fn test_rename_stop() {
        let mut network = BusNetwork::new();
        network.add_or_get_stop("Stop 1");
        network.rename_stop("Stop 1", "New Stop");
        assert!(network.get_stop("Stop 1").is_none());
        assert!(network.get_stop("New Stop").is_some());
    }
}
