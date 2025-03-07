use deutsche_bahn_delay_reasons::Grund as DbGrund;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Grund {
    FahrtFaelltAus,
    FaehrtHeuteNurBis(String),
    VerzoegerungenImBetriebsablauf,
    Gleiswechsel,
    FahrtFaelltAusMitErsatzfahrt(String),
    TechnischeStoerungenAmZug,
    VerspaetungEinesVorausfahrendenZuges,
    VerspaeteteBereitstellungDesZuges,
    Bauarbeiten,
    WitterungsbedingteStoerung,
    WeichenStoerung,
    AenderungImFahrtverlauf,
    Unwetter,
}

impl From<DbGrund> for Grund {
    fn from(grund: DbGrund) -> Self {
        match grund {
            DbGrund::FahrtFaelltAus => Grund::FahrtFaelltAus,
            DbGrund::FaehrtHeuteNurBis(s) => Grund::FaehrtHeuteNurBis(s),
            DbGrund::VerzoegerungenImBetriebsablauf => Grund::VerzoegerungenImBetriebsablauf,
            DbGrund::Gleiswechsel => Grund::Gleiswechsel,
            DbGrund::FahrtFaelltAusMitErsatzfahrt(s) => Grund::FahrtFaelltAusMitErsatzfahrt(s),
            DbGrund::TechnischeStoerungenAmZug => Grund::TechnischeStoerungenAmZug,
            DbGrund::VerspaetungEinesVorausfahrendenZuges => {
                Grund::VerspaetungEinesVorausfahrendenZuges
            }
            DbGrund::VerspaeteteBereitstellungDesZuges => Grund::VerspaeteteBereitstellungDesZuges,
            DbGrund::Bauarbeiten => Grund::Bauarbeiten,
            DbGrund::WitterungsbedingteStoerung => Grund::WitterungsbedingteStoerung,
            DbGrund::WeichenStoerung => Grund::WeichenStoerung,
            DbGrund::AenderungImFahrtverlauf => Grund::AenderungImFahrtverlauf,
            DbGrund::Unwetter => Grund::Unwetter,
            _ => Grund::VerzoegerungenImBetriebsablauf, // fallback for non_exhaustive
        }
    }
}

impl fmt::Display for Grund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grund::FahrtFaelltAus => write!(f, "Flight cancelled"),
            Grund::FaehrtHeuteNurBis(s) => write!(f, "Flight only goes to {}", s),
            Grund::VerzoegerungenImBetriebsablauf => write!(f, "Operational delays"),
            Grund::Gleiswechsel => write!(f, "Gate change"),
            Grund::FahrtFaelltAusMitErsatzfahrt(s) => {
                write!(f, "Flight cancelled, replacement flight {}", s)
            }
            Grund::TechnischeStoerungenAmZug => write!(f, "Technical difficulties with aircraft"),
            Grund::VerspaetungEinesVorausfahrendenZuges => {
                write!(f, "Delay due to previous flight")
            }
            Grund::VerspaeteteBereitstellungDesZuges => write!(f, "Delayed aircraft preparation"),
            Grund::Bauarbeiten => write!(f, "Runway maintenance"),
            Grund::WitterungsbedingteStoerung => write!(f, "Weather-related delay"),
            Grund::WeichenStoerung => write!(f, "Ground equipment malfunction"),
            Grund::AenderungImFahrtverlauf => write!(f, "Flight path changes"),
            Grund::Unwetter => write!(f, "Severe weather conditions"),
        }
    }
}

pub fn get_random_grund() -> Grund {
    deutsche_bahn_delay_reasons::get_grund().into()
}
