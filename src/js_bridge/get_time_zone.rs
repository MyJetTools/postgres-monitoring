use crate::models::*;

pub fn get_time_zone() -> TimeZone {
    let times = dioxus_utils::js::eval("new Date().getTimezoneOffset()");
    let time_zone = times.as_f64().unwrap() as i64;
    time_zone.into()
}
