mod public_transport;
mod transport_network;

fn main() {
    println!("========= BUS NETWORK =========");
    bus_stop();

    println!("\n========= TRAIN NETWORK =========");
    train_stop();
}


fn bus_stop() {
    use public_transport::BusNetwork;

    let mut network = BusNetwork::new();

    // Add three stops to the network.
    network.add_or_get_stop("Vienna");
    network.add_or_get_stop("Grz");
    network.add_or_get_stop("Linz");
    network.dump(
        "Ref count of all stops is 1 because each stop is only referenced once by the network",
    );

    network.add_route("Vienna", "Grz");
    network.dump(
        "Ref count to Graz should be 2 because it is referenced by the network and by Vienna",
    );
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

fn train_stop() {
    use transport_network::{Named, TransportNetwork};

    #[derive(Debug)]
    struct TrainStop {
        name: String,
        _has_ticket_machine: bool,
        _has_shelter: bool,
    }

    impl Named for TrainStop {
        fn name(&self) -> &str {
            &self.name
        }

        fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }
    }

    let mut network = TransportNetwork::<TrainStop>::new();

    // Add three stops to the network.
    network.add_stop(TrainStop {
        name: "Vienna".to_string(),
        _has_ticket_machine: true,
        _has_shelter: true,
    });
    network.add_stop(TrainStop {
        name: "Grz".to_string(),
        _has_ticket_machine: false,
        _has_shelter: true,
    });
    network.add_stop(TrainStop {
        name: "Linz".to_string(),
        _has_ticket_machine: true,
        _has_shelter: false,
    });
    network.dump(
        "Ref count of all stops is 1 because each stop is only referenced once by the network",
    );

    network.add_route("Vienna", "Grz");
    network.dump(
        "Ref count to Graz should be 2 because it is referenced by the network and by Vienna",
    );
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