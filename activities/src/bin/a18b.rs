// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


// * Use an enum to represent all types of employees
enum Position {
    Maintenance,
    Marketing,
    Managers,
    LineSuper,
    Kitchen,
    AssemblyTech
}

enum Status {
    Active,
    Terminated
}

// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    employee_type: Position,
    status: Status
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn enter_building (employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Employee Terminated, cannot enter".to_owned()),
        _ => (),
    }

    match employee.employee_type {
        Position::Maintenance => Ok(()),
        Position::Managers => Ok(()),
        Position::Marketing => Ok(()),
        _ => Err("Invalid Position".to_owned())
    }
}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access (employee: &Employee) -> Result<(), String> {
    let attempt_access = enter_building(employee)?;
    println!("access ok");
    Ok(())
}


fn main() {

    let bob = Employee {
        employee_type: Position::LineSuper,
        status: Status::Active
    };

    match print_access(&bob) {
        Err(e) => println!("Access denied {:?}", e),
        _ => ()
    };

}
