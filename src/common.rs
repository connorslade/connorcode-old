use afire::{internal::common::remove_address_port, Header, Request};

const FILE_SIZES: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
const TIME_UNITS: &[(&str, u16)] = &[
    ("second", 60),
    ("minute", 60),
    ("hour", 24),
    ("day", 30),
    ("month", 12),
    ("year", 0),
];

pub fn get_header(headers: Vec<Header>, header: &str) -> Option<String> {
    for i in headers {
        if i.name == header {
            return Some(i.value);
        }
    }
    None
}

/// Convert a Byte size into the biggest unit
pub fn best_size(bytes: u64) -> String {
    let mut bytes = bytes as f64;

    for i in FILE_SIZES {
        if bytes < 1024.0 {
            return format!("{} {}", (bytes * 10.0).round() / 10.0, i);
        }
        bytes /= 1024.0;
    }

    format!(
        "{} {}",
        (bytes * 10.0).round() / 10.0,
        FILE_SIZES.last().expect("File Size Array is Empty!")
    )
}

pub fn best_time(secs: u64) -> String {
    let mut secs = secs as f64;

    for i in TIME_UNITS {
        if i.1 == 0 || secs < i.1 as f64 {
            secs = secs.round();
            return format!("{} {}{}", secs, i.0, if secs > 1.0 { "s" } else { "" });
        }

        secs /= i.1 as f64;
    }

    format!("{} years", secs.round())
}

pub fn get_ip(req: &Request) -> String {
    let mut ip = remove_address_port(&req.address);
    if ip == "127.0.0.1" {
        if let Some(i) = req.headers.iter().find(|x| x.name == "X-Forwarded-For") {
            ip = i.value.to_owned();
        }
    }

    ip
}
