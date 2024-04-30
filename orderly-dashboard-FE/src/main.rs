use indexer::Events;
use query_service::Api;
use std::fs;
use typescript_type_def::write_definition_file;

fn main() {
    let mut buf = Vec::new();
    write_definition_file::<_, (Api, Events)>(&mut buf, Default::default()).unwrap();
    let types = String::from_utf8(buf).unwrap();
    fs::write("./app/types/api.ts", types).unwrap();
}
