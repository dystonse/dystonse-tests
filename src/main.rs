use gtfs_structures::Gtfs;
use std::error::Error;

// This is handy, because mysql defines its own Result type and we don't
// want to repeat std::result::Result
type FnResult<R> = std::result::Result<R, Box<dyn Error>>;

/// a crate for various testing purposes, 
/// so we don't have to mess around with our big actual project code 
/// to quickly test simple new features.
fn main() -> FnResult<()> {
    // just comment out any unwanted tests:
    route_variant_test()?;
    Ok(())
}

const ROUTE_ID : &str = "53837_3";
//const ROUTE_ID : &str = "vrn-8-10b-1";
const SCHEDULE_PATH : &str = "data/test_schedule_vbn.zip";
//const SCHEDULE_PATH : &str = "data/test_schedule_vrn.zip";

/// used to see if the trips in a route have their shape_id and/or route_variant set correctly
/// (note: vbn provides shape_id values, while vrn doesn't.)
fn route_variant_test() -> FnResult<()> {
    println!("testing shape_id and route_variant computation:");
    println!("parsing schedule...");
    let schedule = Gtfs::new(SCHEDULE_PATH)?;
    println!("schedule parsed successfully.");
    println!("start looking up route {} ...", ROUTE_ID);
    for trip in schedule.trips.values() {
        if trip.route_id == ROUTE_ID.to_string() {
            let shape_id = match &trip.shape_id {
                Some(s) => s,
                None => "(none)"
            };
            let rv = match &trip.route_variant {
                Some(r) => r,
                None => "(none)"
            };
            println!("Found trip {} with shape_id {} and route_variant {}", trip.id, shape_id, rv);
        }
    }

    println!("Done :)");
    Ok(())
}