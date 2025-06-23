use chrono;

pub fn handle() -> String {
    chrono::Local::now().format("%d/%m/%Y %T").to_string()
}
