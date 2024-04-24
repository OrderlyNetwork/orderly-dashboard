use std::fs;

use query_service::Api;
use typescript_type_def::write_definition_file;

fn main() {
    let mut buf = Vec::new();
    write_definition_file::<_, Api>(&mut buf, Default::default()).unwrap();
    let types = String::from_utf8(buf).unwrap();
    fs::write("./orderly-dashboard-FE/app/types/api.ts", types).unwrap();
}
