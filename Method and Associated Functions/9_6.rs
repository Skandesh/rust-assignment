
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str{
        match *self{
            TrafficLightColor::Red =>"red",
            TrafficLightColor::Yellow =>"yellow",
            TrafficLightColor::Green =>"green"
        }
        

    }
    
}

fn main() {
    let y = TrafficLightColor::Yellow;
    let r = TrafficLightColor::Red;
    let g = TrafficLightColor::Green;
    
    assert_eq!(y.color(), "yellow");
   

    println!("{:?}",y);
    println!("{:?}",r.color());
    println!("{:?}",g.color());
}
