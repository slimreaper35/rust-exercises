use chrono::Datelike;
use itertools::Itertools;

use crate::parsing::data_access;
use crate::parsing::datasets::traffic_accident::enums;

/// Encode the trend of accidents with unbuckled vehicle occupants over year-long windows into a String.
pub fn unbuckled_trend() -> String {
    let accidents = data_access::DataAccess::accidents();

    let frequencies = accidents
        .iter()
        .filter(|accident| {
            accident.participant_vehicles.iter().any(|vehicle| {
                vehicle.occupants.iter().any(|person| {
                    !matches!(
                        person.person_detail,
                        Some(enums::PersonDetail::WearingASeatbelt)
                    )
                })
            })
        })
        .map(|item| item.overview.date.year())
        .sorted()
        .group_by(|&year| year);

    frequencies
        .into_iter()
        .map(|(_, group)| group.count())
        .tuple_windows()
        .map(|(a, b)| match a.cmp(&b) {
            std::cmp::Ordering::Less => '^',
            std::cmp::Ordering::Equal => '=',
            std::cmp::Ordering::Greater => 'v',
        })
        .collect()
}
