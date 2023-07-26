use std::cell::RefCell;
use std::rc::Rc;

type TransportStopRef<T> = Rc<RefCell<TransportStop<T>>>;

/// Every stop in a transport network must have a name.
pub trait Named {
    /// Returns the name of the stop.
    fn name(&self) -> &str;

    /// Sets the name of the stop.
    fn set_name(&mut self, name: &str);
}

/// A stop in a public transport network.
#[derive(Debug, Clone)]
pub struct TransportStop<T: Named> {
    /// The user data associated with this stop.
    data: T,

    /// The destinations reachable from this stop.
    destinations: Vec<TransportStopRef<T>>,
}

impl<T: Named> TransportStop<T> {
    /// Creates a new stop with the given user data.
    fn new(data: T) -> TransportStopRef<T> {
        Rc::new(RefCell::new(TransportStop {
            data,
            destinations: Default::default(),
        }))
    }

    /// Adds a new destination reachable from this stop.
    fn add_edge(&mut self, target: TransportStopRef<T>) {
        self.destinations.push(target);
    }
}

/// If the user data associated with a stop implements `Named`, the stop also implements `Named`.
impl<T: Named> Named for TransportStop<T> {
    fn name(&self) -> &str {
        self.data.name()
    }

    fn set_name(&mut self, name: &str) {
        self.data.set_name(name);
    }
}

/// A public transport network.
#[derive(Debug)]
pub struct TransportNetwork<T: Named> {
    stops: Vec<TransportStopRef<T>>,
}

impl<T: Named> TransportNetwork<T> {
    /// Creates a new, empty public transport network.
    pub fn new() -> TransportNetwork<T> {
        TransportNetwork {
            stops: Default::default(),
        }
    }

    /// Returns the stop with the given name, if it exists.
    pub fn get_stop(&self, name: &str) -> Option<TransportStopRef<T>> {
        self.stops
            .iter()
            .find(|stop| stop.borrow().name() == name)
            // Clone the reference to the stop if it exists.
            .map(Clone::clone)
    }
    
    /// Adds a new stop to the network.
    pub fn add_stop(&mut self, data: T) -> TransportStopRef<T> {
        // TODO: Check if a stop with the given name already exists. Not done here to keep the example simple.
        let new_stop = TransportStop::new(data);
        // Add a clone of the reference to the new stop to the network.
        self.stops.push(Clone::clone(&new_stop));
        new_stop
    }

    /// Adds a route between two stops.
    pub fn add_route(&mut self, source_name: &str, target_name: &str) {
        let source = self.get_stop(source_name);
        let target = self.get_stop(target_name);
        if let (Some(source), Some(target)) = (source, target) {
            source.borrow_mut().add_edge(target);
        }

        // TODO: Return error indicator if source or target don't exist. Not done here to keep the example simple.
    }

    /// Returns the destinations reachable from the given stop.
    /// If the stop doesn't exist, an empty vector is returned.
    pub fn destinations(&self, source_name: &str) -> Vec<TransportStopRef<T>> {
        let source_stop = self.get_stop(source_name);
        if let Some(source_stop) = source_stop {
            source_stop.borrow().destinations.clone()
        } else {
            Vec::new()
        }
    }

    /// Renames a stop.
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
            println!("Stop {} ({})", stop.borrow().name(), Rc::strong_count(stop));
        }
    }
}
