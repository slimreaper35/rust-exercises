use crate::parsing::data_access;
use crate::parsing::datasets::traffic_accident::enums;

/// Search for the least favorable accident on record, one where the odds were stacked against the drivers.
pub fn unfavorable_accident() -> Option<String> {
    let accidents = data_access::DataAccess::accidents();

    let mut very_bad_accidents = accidents.iter().filter(|accident| {
        accident.overview.road_condition != enums::RoadCondition::DryAndUncontaminated
            && accident.overview.weather_condition != enums::WeatherCondition::Favorable
            && accident.overview.visibility != enums::Visibility::DaytimeVisibilityNormal
            && accident.overview.visibility
                != enums::Visibility::NighttimeVisibilityWithStreetLightsNormal
            && accident.overview.visibility
                != enums::Visibility::NighttimeVisibilityWithoutStreetLightsNormal
            && accident.overview.drivers_view != enums::DriversView::Unobstructed
            && accident.overview.alcohol != enums::AlcoholCondition::None
            && accident.overview.alcohol != enums::AlcoholCondition::NotInvestigated
            && accident.overview.alcohol_in_culprit == enums::AlcoholInCulprit::Yes
            && accident.participant_vehicles.iter().all(|vehicle| {
                vehicle.driver_condition != Some(enums::DriverCondition::Good)
                    && vehicle.driver_influenced != Some(enums::DriverInfluenced::NoInfluence)
            })
    });

    let worst_accident = very_bad_accidents.next()?;

    let date = worst_accident
        .overview
        .date
        .format("%d.%_m. %Y")
        .to_string();

    let time = match worst_accident.overview.time {
        enums::AccidentTime::Exact(time) => time.format("%k:%M").to_string(),
        enums::AccidentTime::Hour(hour) => format!("{}:??", hour),
        _ => String::from("??:??"),
    };

    Some(format!("{}{}", date, time))
}
