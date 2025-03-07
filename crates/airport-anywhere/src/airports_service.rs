use sky_tracer::model::airport::{Airport, AirportError};
use std::collections::HashMap;
use std::sync::Arc;

const AIRPORTS_DATA: &str = include_str!("../../../assets/airports.dat");

/// Service for looking up airports
#[derive(Debug, Clone)]
pub struct AirportsService {
    airports_by_code: HashMap<String, Arc<Airport>>,
}

impl AirportsService {
    /// Creates a new AirportsService by parsing CSV data
    pub fn from_csv_str(data: &str) -> Result<Self, AirportError> {
        let airports = crate::data_loader::load_airports_from_csv(data)?;
        let mut airports_by_code = HashMap::new();

        for airport in airports {
            if !airport.code.is_empty() {
                airports_by_code.insert(airport.code.clone(), airport);
            }
        }

        Ok(Self { airports_by_code })
    }

    /// Gets instance of AirportsService
    pub fn instance() -> Result<Self, AirportError> {
        Self::from_csv_str(AIRPORTS_DATA)
    }

    /// Find an airport by its code (IATA or combined IATA/ICAO)
    pub fn find_by_code(&self, code: &str) -> Result<Arc<Airport>, AirportError> {
        println!("Searching for airport with code: {}", code); // Debug log

        // First try exact match
        if let Some(airport) = self.airports_by_code.get(code) {
            return Ok(airport.clone());
        }

        // If no exact match, try matching just the IATA part
        self.airports_by_code
            .values()
            .find(|airport| airport.code.starts_with(code))
            .cloned()
            .ok_or_else(|| {
                println!("Airport not found with code: {}", code); // Debug log
                AirportError::NotFound(code.to_string())
            })
    }

    /// Get all airports
    pub fn all(&self) -> impl Iterator<Item = &Arc<Airport>> {
        self.airports_by_code.values()
    }

    /// Search airports by name (case-insensitive partial match)
    pub fn search_by_name(&self, query: &str) -> Vec<Arc<Airport>> {
        let query = query.to_lowercase();
        self.airports_by_code
            .values()
            .filter(|airport| airport.name.to_lowercase().contains(&query))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let data = r#"1,"Test Airport","Test City","Test Country","TST","TTST",12.345,-67.890,1234,2,"E","Europe/Test","airport","Test Source""#;
        let service = AirportsService::from_csv_str(data).unwrap();

        let airport = service.find_by_code("TST/TTST").unwrap();
        assert_eq!(airport.name, "Test Airport");
        assert_eq!(airport.code, "TST/TTST");
    }

    #[test]
    fn test_search_by_name() {
        let data = r#"1,"Frankfurt Airport","City1","Country1","FRA","EDDF",50.033,8.571,0,1,"E","Europe/Berlin","airport","test"
2,"Frankfurt-Hahn","City2","Country2","HHN","EDFH",49.945,7.264,0,1,"E","Europe/Berlin","airport","test""#;

        let service = AirportsService::from_csv_str(data).unwrap();
        let results = service.search_by_name("frankfurt");

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|a| a.code == "FRA/EDDF"));
        assert!(results.iter().any(|a| a.code == "HHN/EDFH"));
    }

    #[test]
    fn test_not_found() {
        let data = r#"1,"Test Airport","Test City","Test Country","TST","TTST",12.345,-67.890,1234,2,"E","Europe/Test","airport","Test Source""#;
        let service = AirportsService::from_csv_str(data).unwrap();

        assert!(service.find_by_code("NONEXISTENT").is_err());
    }

    #[test]
    fn test_all_airports() {
        let data = r#"1,"Airport1","City1","Country1","AA1","AAA1",1.0,1.0,0,0,"E","UTC","airport","test"
2,"Airport2","City2","Country2","AA2","AAA2",2.0,2.0,0,0,"E","UTC","airport","test""#;

        let service = AirportsService::from_csv_str(data).unwrap();
        let airports: Vec<_> = service.all().collect();

        assert_eq!(airports.len(), 2);
        assert!(airports.iter().any(|a| a.code == "AA1/AAA1"));
        assert!(airports.iter().any(|a| a.code == "AA2/AAA2"));
    }
}
