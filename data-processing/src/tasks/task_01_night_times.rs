use crate::parsing::data_access;
use crate::parsing::datasets::traffic_accident::enums;
use chrono::NaiveTime;

/// What range of timestamps do accidents which are considered to have happened during the night fall into?
pub fn night_times() -> (NaiveTime, NaiveTime) {
    let accidents = data_access::DataAccess::accidents();
    let night_accidents = accidents
        .iter()
        .filter(|x| x.overview.time_of_day == enums::TimeOfDay::Nighttime);

    let latest_absolute = NaiveTime::from_hms_opt(23, 59, 0).unwrap();
    let mid = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
    let earliest_absolute = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    let (start, end) = night_accidents.fold(
        (latest_absolute, earliest_absolute),
        |(current_latest, current_earliest), accident| match accident.overview.time {
            enums::AccidentTime::Exact(time) => {
                if time < current_latest && time > mid {
                    (time, current_earliest)
                } else if time > current_earliest && time < mid {
                    (current_latest, time)
                } else {
                    (current_latest, current_earliest)
                }
            }
            _ => (current_latest, current_earliest),
        },
    );

    (start, end)
}
