//! Utility functions for the project

/// Initialize the logger with the given log level
///
/// # Arguments
///
/// * `lvl` - The log level to set the logger to
///
/// # Example
///
/// ```
/// use log::LevelFilter;
///
/// init_logger(LevelFilter::Debug);
pub fn init_logger(lvl: log::LevelFilter) {
    env_logger::builder().filter_level(lvl).init();
}

#[allow(dead_code)]
pub enum HorizontalLineType {
    Single,
    Double,
}

/// Add a horizontal line to the given text body to separate sections
pub async fn add_horizontal_line(text_body: &mut String, hline_config: HorizontalLineType) {
    let line_length = match hline_config {
        HorizontalLineType::Single => 40,
        HorizontalLineType::Double => 80,
    };

    let line_char = match hline_config {
        HorizontalLineType::Single => '-',
        HorizontalLineType::Double => '=',
    };

    for _ in 0..line_length {
        text_body.push(line_char);
    }

    text_body.push('\n');
}
