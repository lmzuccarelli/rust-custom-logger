use chrono::{DateTime, Local};

// logging convenience functions
#[derive(Eq, PartialEq)]
pub enum Level {
    INFO,
    DEBUG,
    TRACE,
    WARN,
}

//#[derive(Default)]
pub struct Logging {
    pub log_level: Level,
}

impl Logging {
    // info
    pub fn info(&self, msg: &str) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            let dt = Local::now();
            let naive_utc = dt.naive_utc();
            let offset = dt.offset().clone();
            let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
            let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "\x1b[1;94m [ {} {} ] \x1b[0m  : {}",
                "INFO ", dt_formated, msg
            );
        }
    }
    /// debug
    pub fn debug(&self, msg: &str) {
        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
        if self.log_level == Level::DEBUG || self.log_level == Level::TRACE {
            println!(
                "\x1b[1;92m [ {} {} ] \x1b[0m  : {}",
                "DEBUG", dt_formated, msg
            );
        }
    }
    /// info with highlight
    pub fn hi(&self, msg: &str) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            let dt = Local::now();
            let naive_utc = dt.naive_utc();
            let offset = dt.offset().clone();
            let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
            let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "\x1b[1;94m [ {}  {} ] \x1b[0m  : \x1b[1;93m{} \x1b[0m",
                "INFO", dt_formated, msg
            );
        }
    }
    /// info with mid level highlight
    pub fn mid(&self, msg: &str) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            let dt = Local::now();
            let naive_utc = dt.naive_utc();
            let offset = dt.offset().clone();
            let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
            let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "\x1b[1;94m [ {}  {} ]  \x1b[0m : \x1b[1;94m{} \x1b[0m",
                "INFO", dt_formated, msg
            );
        }
    }
    // info with low level highlight
    pub fn lo(&self, msg: &str) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            let dt = Local::now();
            let naive_utc = dt.naive_utc();
            let offset = dt.offset().clone();
            let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
            let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "\x1b[1;94m [ {}  {} ]  \x1b[0m : \x1b[1;95m{} \x1b[0m",
                "INFO", dt_formated, msg
            );
        }
    }
    // info with extra level highlight
    pub fn ex(&self, msg: &str) {
        if self.log_level == Level::INFO
            || self.log_level == Level::DEBUG
            || self.log_level == Level::TRACE
        {
            let dt = Local::now();
            let naive_utc = dt.naive_utc();
            let offset = dt.offset().clone();
            let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
            let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
            println!(
                "\x1b[1;94m [ {}  {} ]  \x1b[0m : \x1b[1;98m{} \x1b[0m",
                "INFO", dt_formated, msg
            );
        }
    }
    /// trace
    pub fn trace(&self, msg: &str) {
        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
        if self.log_level == Level::TRACE {
            println!(
                "\x1b[1;96m [ {} {} ] \x1b[0m  : {}",
                "TRACE", dt_formated, msg
            );
        }
    }
    /// warning
    pub fn warn(&self, msg: &str) {
        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
        println!(
            "\x1b[1;93m [ {}  {} ] \x1b[0m  : {}",
            "WARN", dt_formated, msg
        );
    }
    /// error
    pub fn error(&self, msg: &str) {
        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S%.3f");
        println!(
            "\x1b[1;91m [ {} {} ] \x1b[0m  : {}",
            "ERROR", dt_formated, msg
        );
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_info_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.info("testing info logging");
    }

    #[test]
    fn test_debug_pass() {
        let log = &Logging {
            log_level: Level::DEBUG,
        };
        log.debug("testing debug logging");
    }

    #[test]
    fn test_trace_pass() {
        let log = &Logging {
            log_level: Level::TRACE,
        };
        log.trace("testing trace logging");
    }

    #[test]
    fn test_warn_pass() {
        let log = &Logging {
            log_level: Level::WARN,
        };
        log.warn("testing warn logging");
    }

    #[test]
    fn test_mid_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.mid("testing mid logging");
    }

    #[test]
    fn test_hi_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.hi("testing hi logging");
    }

    #[test]
    fn test_lo_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.lo("testing lo logging");
    }

    #[test]
    fn test_ex_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.ex("testing ex logging");
    }

    #[test]
    fn test_error_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.error("testing error logging");
    }

    // set level to TRACE
    #[test]
    fn test_mid_with_trace_pass() {
        let log = &Logging {
            log_level: Level::TRACE,
        };
        log.mid("testing mid logging with TRACE");
    }

    #[test]
    fn test_hi_with_trace_pass() {
        let log = &Logging {
            log_level: Level::TRACE,
        };
        log.hi("testing hi logging with TRACE");
    }
}
