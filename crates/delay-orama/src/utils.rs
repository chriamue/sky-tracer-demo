pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Earth's radius in kilometers
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin()
        + lat1.to_radians().cos()
            * lat2.to_radians().cos()
            * (dlon / 2.0).sin()
            * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c
}
