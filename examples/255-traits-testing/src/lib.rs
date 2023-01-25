#![allow(dead_code)]

#[cfg(test)]
use mockall::automock;

struct Customer {
    id: usize,
    name: String,
}

#[cfg_attr(test, automock)]
trait CustomerRepository {
    fn get(&self, key: usize) -> Option<Customer>;
}

struct CustomerDatabaseRepository {}

impl CustomerRepository for CustomerDatabaseRepository {
    fn get(&self, key: usize) -> Option<Customer> {
        // Here we would implement the DB logic to retrieve a customer
        match key {
            0 => Some(Customer {
                id: key,
                name: "John Doe".to_string(),
            }),
            _ => None,
        }
    }
}

struct BusinessLogic<'a> {
    customer_repository: &'a dyn CustomerRepository,
}

impl<'a> BusinessLogic<'a> {
    fn count_johns(&self) -> usize {
        let mut count = 0;
        for i in 0..10 {
            let customer = self.customer_repository.get(i);
            if customer.is_some() && customer.unwrap().name == "John Doe" {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::{gt, lt};

    use super::*;

    #[test]
    fn count_johns() {
        let mut repo_mock = MockCustomerRepository::new();
        repo_mock.expect_get()
            .with(lt(2))
            .returning(|_| Some(Customer {
                id: 0,
                name: "John Doe".to_string(),
            }));
        repo_mock.expect_get()
            .with(gt(0))
            .returning(|_| None);

        let business_logic = BusinessLogic { customer_repository: &repo_mock };
        assert_eq!(business_logic.count_johns(), 2);
    }
}
