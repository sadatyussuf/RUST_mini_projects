const CAR_PRODUCED: f64 = 221.0;
pub fn production_rate_per_hour(speed: u8) -> f64 {  
    CAR_PRODUCED * (speed as f64) * error_rate(speed)
}
pub fn working_items_per_minute(speed: u8) -> u32 {
    let working_cars = production_rate_per_hour(speed);
    return ((working_cars as u32) / 60) as u32
        
}
pub fn error_rate(speed:u8)->f64{
       match speed{ 
        1..=4 => 1.0, 
        5..=8 => 0.9,   
        9 | 10 => 0.77,    
        _ =>  0.0
    } 
}