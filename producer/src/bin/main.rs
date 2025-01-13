use anyhow::Result;
use fake::faker::automotive::fr_fr::LicencePlate;
use fake::faker::company::en::CompanyName;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::{FirstName, LastName};
use fake::Fake;
use rand::Rng;
use tanit::application::http::{HttpServer, HttpServerConfig};
use tanit::domain::models::{Car, Ferri, Passenger};
use uuid::Uuid;

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

fn generate_random_ferri() -> Ferri {
    Ferri {
        id: generate_id(),                               // Random UUID
        name: CompanyName().fake(), // Random ferry name (company name as placeholder)
        capacity: rand::thread_rng().gen_range(50..200), // Random capacity between 50 and 300
    }
}

fn generate_random_car() -> Car {
    let mut rng = rand::thread_rng();
    Car {
        id: generate_id(),                    // Random UUID
        licence_plate: LicencePlate().fake(), // Random license plate
        brand: Word().fake(),                 // Random car brand (word as placeholder)
        color: Word().fake(),                 // Random car color (word as placeholder)
        capacity: rng.gen_range(2..5),        // Random capacity between 2 and 8
    }
}

fn generate_random_passenger(ferri_id: &str, car_id: Option<&str>) -> Passenger {
    let mut rng = rand::thread_rng();
    Passenger {
        id: generate_id(),                       // Random UUID
        car_id: car_id.map(|id| id.to_string()), // Optional car ID (Some or None)
        ferri_id: ferri_id.to_string(),          // Assigned ferry ID
        firstname: FirstName().fake(),           // Random first name
        lastname: LastName().fake(),             // Random last name
        sex: rng.gen_bool(0.5),                  // Random sex (true or false)
    }
}

fn generate_random_data() {
    // Generate a random ferry
    let ferri = generate_random_ferri();
    println!("Ferri: {:#?}", ferri);

    // Generate a random car
    let car = generate_random_car();
    println!("Car: {:#?}", car);

    // Generate a random passenger associated with the ferry and optionally the car
    let passenger_with_car = generate_random_passenger(&ferri.id, Some(&car.id));
    let passenger_without_car = generate_random_passenger(&ferri.id, None);

    println!("Passenger with car: {:#?}", passenger_with_car);
    println!("Passenger without car: {:#?}", passenger_without_car);
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    generate_random_data();

    let server_config = HttpServerConfig::new("3333".to_string());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
