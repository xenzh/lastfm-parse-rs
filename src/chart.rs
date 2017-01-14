use std::convert::Into;
use super::common::Wrapped;


/// api methods: tag.getweeklychartlist
wrapper_t!(WeeklyChartList, weeklychartlist, Weekly);


#[derive(Deserialize, Debug)]
pub struct WeeklyEntry {
    pub from: u32,
    pub to: u32,
}

#[derive(Deserialize, Debug)]
pub struct Weekly {
    chart: Option<Vec<WeeklyEntry>>,
}