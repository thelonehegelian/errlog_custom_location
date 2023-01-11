use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() {
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("log_file")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("The Humans are Dead!");
}
