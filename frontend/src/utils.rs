use chrono::{DateTime, Local};

pub fn format_display_price(price: u32) -> String {
    format!("{}.{:02}€", price / 100, price % 100)
}

pub fn format_datetime(datetime: DateTime<Local>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn class_if(cond: bool, class: &str) -> Option<&str> {
    if cond {
        Some(class)
    } else {
        None
    }
}
