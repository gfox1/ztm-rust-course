// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    // We need to access all values in stuctures with functions when using triaits
    fn cost_per_sq_meter (&self) -> f64;
    fn sq_meters (&self) -> f64;
    fn total_cost (&self) -> f64 {
        self.cost_per_sq_meter() * self.sq_meters()
    }
}

// Tuple structure so we are accessing the first feild with .0
struct Carpet(f64);
impl Material for Carpet{
    // We need to access all values in stuctures with functions when using triaits
    // This is why we have a fn that returns 10
    fn cost_per_sq_meter (&self) -> f64 {
        10.0
    }
    fn sq_meters (&self) -> f64 {
        self.0
    }
    // dont need to include total cost fn becuase we have defined a generic one in the materials trait
}
struct Tile(f64);
impl Material for Tile{
    fn cost_per_sq_meter (&self) -> f64 {
        15.0
    }
    fn sq_meters (&self) -> f64 {
        self.0
    }
}
struct Wood(f64);
impl Material for Wood{
    fn cost_per_sq_meter (&self) -> f64 {
        20.0
    }
    fn sq_meters (&self) -> f64 {
        self.0
    }
}

fn total_material_cost (materials: &Vec<Box<dyn Material>>) -> f64 {
    materials.iter().map(|materials| materials.total_cost()).sum()
}

fn main() {
    let carpet = Box::new(Carpet(20.0));
    let tile = Box::new(Tile(10.0));
    let wood = Box::new(Wood(30.0));

    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    let total = total_material_cost(&materials);
    println!("Total Cost: ${:?}", total);
}
