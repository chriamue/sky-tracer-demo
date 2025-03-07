use csv::ReaderBuilder;
use sky_tracer::model::airport::Airport;
use sky_tracer::model::airport::AirportError;
use std::sync::Arc;

pub fn load_airports_from_csv(data: &str) -> Result<Vec<Arc<Airport>>, AirportError> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_reader(data.as_bytes());

    let mut airports = Vec::new();
    let mut record = csv::ByteRecord::new();

    while rdr.read_byte_record(&mut record)? {
        let id = parse_field(&record, 0, "id")?;
        let name = parse_utf8(&record, 1, "name")?;
        let iata = parse_utf8(&record, 4, "iata")?.trim().to_string();
        let icao = parse_utf8(&record, 5, "icao")?.trim().to_string();
        let latitude = parse_field(&record, 6, "latitude")?;
        let longitude = parse_field(&record, 7, "longitude")?;

        // Store just the IATA code as the primary code
        let code = if !iata.is_empty() { iata } else { icao.clone() };

        let airport = Airport::new(id, latitude, longitude, name, code);
        airports.push(Arc::new(airport));
    }

    println!("Parsed {} airports", airports.len());
    Ok(airports)
}

fn parse_utf8(record: &csv::ByteRecord, index: usize, field: &str) -> Result<String, AirportError> {
    let bytes = record
        .get(index)
        .ok_or_else(|| AirportError::missing_field(field))?;
    String::from_utf8(bytes.to_vec())
        .map_err(|err| AirportError::invalid_utf8(field, err.to_string()))
}

fn parse_field<T: std::str::FromStr>(
    record: &csv::ByteRecord,
    index: usize,
    field: &str,
) -> Result<T, AirportError> {
    let str_val = parse_utf8(record, index, field)?;
    str_val
        .parse()
        .map_err(|_| AirportError::invalid_value(field, str_val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frankfurt_airport() {
        let data = r#"340,"Frankfurt am Main Airport","Frankfurt","Germany","FRA","EDDF",50.033333,8.570556,364,1,"E","Europe/Berlin","airport","OurAirports""#;

        let airports = load_airports_from_csv(data).unwrap();
        assert_eq!(airports.len(), 1);

        let airport = airports.first().unwrap();
        assert_eq!(airport.code, "FRA");
        assert_eq!(airport.name, "Frankfurt am Main Airport");
        assert_eq!(airport.latitude, 50.033333);
        assert_eq!(airport.longitude, 8.570556);
    }

    #[test]
    fn test_multiple_airports() {
        let data = r#"339,"Erfurt Airport","Erfurt","Germany","ERF","EDDE",50.979801177978516,10.958100318908691,1036,1,"E","Europe/Berlin","airport","OurAirports"
340,"Frankfurt am Main Airport","Frankfurt","Germany","FRA","EDDF",50.033333,8.570556,364,1,"E","Europe/Berlin","airport","OurAirports"
341,"Münster Osnabrück Airport","Munster","Germany","FMO","EDDG",52.134601593,7.68483018875,160,1,"E","Europe/Berlin","airport","OurAirports""#;

        let airports = load_airports_from_csv(data).unwrap();
        assert_eq!(airports.len(), 3);

        assert_eq!(airports[0].code, "ERF");
        assert_eq!(airports[1].code, "FRA");
        assert_eq!(airports[2].code, "FMO");
    }

    #[test]
    fn test_missing_iata() {
        let data = r#"999,"Test Airport","Test City","Test Country","","XXXX",1.0,1.0,100,0,"N","UTC","airport","Test""#;

        let airports = load_airports_from_csv(data).unwrap();
        assert_eq!(airports.len(), 1);

        let airport = airports.first().unwrap();
        assert_eq!(airport.code, "XXXX"); // Should use ICAO when IATA is empty
    }

    #[test]
    fn test_invalid_data() {
        let data = r#"invalid,"Test Airport","Test City","Test Country","TST","TTST",12.345,-67.890,1234,2,"E","Europe/Test","airport","Test Source""#;
        assert!(load_airports_from_csv(data).is_err());
    }
}
