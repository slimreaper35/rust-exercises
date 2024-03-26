use crate::parsing::data_access;

/// Find and format accidents that are somehow extreme:
/// the damages were the highest,
/// the accident involved the most people,
/// the accident involved the most vehicles.
/// Are at least two of them the same accident?
pub fn extreme_accidents() -> (String, bool) {
    let accidents = data_access::DataAccess::accidents();

    let highest_damage = accidents
        .iter()
        .map(|accident| {
            (
                accident.overview.damage_in_czk,
                accident.overview.global_id.as_str(),
            )
        })
        .max_by(|(a_size, _), (b_size, _)| a_size.cmp(b_size));

    let most_people_involved = accidents
        .iter()
        .map(|accident| {
            (
                accident.participant_vehicles.len(),
                accident.overview.global_id.as_str(),
            )
        })
        .max_by(|a, b| a.0.cmp(&b.0));

    let most_vehicles_involved = accidents
        .iter()
        .map(|accident| {
            (
                accident
                    .participant_vehicles
                    .iter()
                    .fold(0, |acc, x| acc + x.occupants.len())
                    + accident.participant_pedestrians.len(),
                accident.overview.global_id.as_str(),
            )
        })
        .max_by(|a, b| a.0.cmp(&b.0));

    let (max_damage, id_1) = highest_damage.unwrap_or((0, ""));
    let (max_vehicles, id_2) = most_people_involved.unwrap_or((0, ""));
    let (max_participants, id_3) = most_vehicles_involved.unwrap_or((0, ""));

    let equal = id_1 == id_2 || id_1 == id_3 || id_2 == id_3;

    (
        format!("{max_damage} Kč : {max_vehicles} vehicles : {max_participants} people"),
        equal,
    )
}
