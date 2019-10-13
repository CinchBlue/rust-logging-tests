#![feature(test)]

#[macro_use] extern crate log;
extern crate simplelog;
extern crate fern;
extern crate log4rs;
extern crate test;
extern crate chrono;

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::config::{Config, Appender, Logger, Root};
    use log::LevelFilter;
    
    fn init_fern() {
        use fern::colors::{ColoredLevelConfig, Color};
        
        let colors = ColoredLevelConfig::new().error(Color::Red).warn(Color::Yellow).info(Color::Cyan).debug(Color::Green).trace(Color::BrightBlack);
        
        let base = fern::Dispatch::new();
        //.level_for("hyper", log::LevelFilter::Warn)
        //.level_for("tokio_reactor", log::LevelFilter::Warn);
        
        let file_cfg = fern::Dispatch::new().level(log::LevelFilter::Info).format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}:{}] [{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.line().map(|x| x.to_string()).unwrap_or("X".to_string()),
                record.level(),
                message
            ))
        }).chain(fern::log_file("fern.log").expect("Failed to create log file!"));
        
        let stdout_cfg = fern::Dispatch::new().level(log::LevelFilter::Info).format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.line().map(|x| x.to_string()).unwrap_or("X".to_string()),
                record.level(),
                message
            ))
        }).chain(std::io::stdout());
        
        base.chain(file_cfg).chain(stdout_cfg).apply().expect("Failed to setup logging!");
    }
    
    fn init_log4rs() {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] [{M}:{L}] [{h({l})}] {m}{n}")))
            .build();
        
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] [{M}:{L}] [{h({l})}] {m}{n}")))
            .build("log4rs.log")
            .unwrap();
        
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .build(Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info))
            .unwrap();
        
        let handle = log4rs::init_config(config).unwrap();
    }
    
    fn log_single_info_line() {
        info!("{}", test::black_box("test 1 2 3"));
    }
    
    fn log_single_trace_line() {
        trace!("{}", test::black_box("test 1 2 3"));
    }
    
    fn log_multi_info_lines() {
        info!("{}", test::black_box("test 1 2 3"));
        info!("{}", test::black_box("test 4 5 6"));
        info!("{}", test::black_box("test 7 8 9"));
    }
    
    fn log_multi_types_lines() {
        trace!("{}", test::black_box("test 1 2 3"));
        debug!("{}", test::black_box("test 4 5 6"));
        info!("{}", test::black_box("test 7 8 9"));
    }
    
    #[bench]
    fn bench_fern_single_unfiltered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_fern();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_single_info_line);
    }
    
    #[bench]
    fn bench_fern_single_filtered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_fern();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_single_trace_line);
    }
    
    #[bench]
    fn bench_fern_multi_filtered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_fern();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_multi_types_lines);
    }
    
    #[bench]
    fn bench_fern_multi_unfiltered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_fern();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_multi_info_lines);
    }
    
    #[bench]
    fn bench_log4rs_single_unfiltered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_log4rs();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_single_info_line);
    }
    
    #[bench]
    fn bench_log4rs_single_filtered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_log4rs();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_single_trace_line);
    }
    
    #[bench]
    fn bench_log4rs_multi_filtered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_log4rs();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_multi_types_lines);
    }
    
    #[bench]
    fn bench_log4rs_multi_unfiltered_line(b: &mut Bencher) {
        // Given: we initialize the config with the correct parameters
        init_log4rs();
        
        // When: we test enough iterations
        // Then: we should get performance numbers
        b.iter(log_multi_info_lines);
    }
}
