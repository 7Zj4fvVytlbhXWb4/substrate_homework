enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Time {
    fn get_time(&self) -> u32;
}

impl Time for TrafficLight {
    fn get_time(&self) -> u32 {
        match *self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 75,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    println!("Red light is {} seconds.", red.get_time());

    let yellow = TrafficLight::Yellow;
    println!("Yellow light is {} seconds.", yellow.get_time());

    let green = TrafficLight::Green;
    println!("Green light is {} seconds.", green.get_time());
}
