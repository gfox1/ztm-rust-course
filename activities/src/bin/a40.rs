// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented
}

#[derive(Debug)]
struct Rental {
    vehicle: Vehicle,
    vin: String,
    status: VehicleStatus

}

struct Corporate (Rc<RefCell<Vec<Rental>>>);

struct StoreFront (Rc<RefCell<Vec<Rental>>>);

fn main() {}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                vehicle: Vehicle::Car,
                vin: "345".to_owned(),
                status: VehicleStatus::Available,

            },
            Rental {
                vehicle: Vehicle::Truck,
                vin: "1239".to_owned(),
                status: VehicleStatus::Maintenance,

            },
            Rental {
                vehicle: Vehicle::Car,
                vin: "r7jty".to_owned(),
                status: VehicleStatus::Rented,
            }
        ];
        let vechicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vechicles));
        let storefront = StoreFront(Rc::clone(&vechicles));
        
        {
            let mut rentals = storefront.0.borrow_mut();
                if let Some (car) = rentals.get_mut(0) {
                    assert_eq!(car.status, VehicleStatus::Available);
                    car.status = VehicleStatus::Rented;
                }
        }
        {
            let mut rentals = corporate.0.borrow_mut();
                if let Some (car) = rentals.get_mut(0) {
                    assert_eq!(car.status, VehicleStatus::Rented);
                    car.status = VehicleStatus::Available;
                }
        }

        let rentals = storefront.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, VehicleStatus::Available);
        }
    }
}