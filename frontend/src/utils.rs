pub fn format_display_price(price: u32) -> String {
    format!("{}.{:02}€", price / 100, price % 100)
}
