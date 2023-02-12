use std::fs;

lazy_static! {
    pub static ref ERROR_PAGE: String = fs::read_to_string("web/dist/template/error.html").unwrap();
    pub static ref WRITING_HOME: String =
        fs::read_to_string("web/dist/template/writing-home.html").unwrap();
    pub static ref WRITING: String = fs::read_to_string("web/dist/template/writing.html").unwrap();
    pub static ref FOOTER: String = fs::read_to_string("web/components/footer.html").unwrap();
}
