use afire::Header;

pub fn get_header(headers: Vec<Header>, header: &str) -> Option<String> {
    for i in headers {
        if i.name == header {
            return Some(i.value);
        }
    }
    None
}
