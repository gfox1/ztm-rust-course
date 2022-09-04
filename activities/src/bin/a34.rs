// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Copy, Clone)]
struct LuggageId(usize);
struct Luggage(LuggageId);

// impl <State> Luggage<State> {
//     fn transition <NextState>(self, state: NextState) -> Luggage<NextState> {
//         Luggage {
//             id: id,
//             state: state
//         }
//     } 
// }

struct CheckIn(LuggageId);
struct OnLoading(LuggageId);
struct Offloading(LuggageId);
struct AwaitingPickup(LuggageId);
struct EndCustody(LuggageId);

impl Luggage {
    fn new (id: LuggageId) -> Self {
        Luggage (id)
    }

    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoading {
        OnLoading(self.0)
    }
}

impl OnLoading {
    fn offload(self) -> Offloading {
        Offloading(self.0)
    }
}

impl Offloading {
    fn await_pickup(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
    
}

fn main() {
    let id = LuggageId(23);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().await_pickup().pickup();
}
