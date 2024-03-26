use crate::parsing::data_access;
use std::collections::HashSet;

/// Which basic settlement units in Brno have never had a recorded accident?
pub fn missing_units() -> HashSet<String> {
    let accidents = data_access::DataAccess::accidents();
    let settlments = data_access::DataAccess::basic_settlement_units();

    let dangerous_settlements: HashSet<String> = accidents
        .iter()
        .map(|accident| accident.overview.basic_settlement_unit.clone())
        .collect();

    let brno_settlements: HashSet<String> = settlments
        .iter()
        .filter(|settlement| settlement.municipality_name == "Brno")
        .map(|settlement| settlement.name.clone())
        .collect();

    let safe_settlements = brno_settlements
        .difference(&dangerous_settlements)
        .cloned()
        .collect();

    safe_settlements
}
