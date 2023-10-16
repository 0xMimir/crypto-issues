use env_logger::Builder;
use std::io::Write;

pub fn setup_logger() {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .filter_module("sqlx::query", log::LevelFilter::Off)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}:{}] {} - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .init();
}
