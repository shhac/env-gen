mod ask;
mod yaml;

fn main() {
    ask::ask("Hello, world!");
    let docs = yaml::read();
    yaml::dump_yaml(docs);
}
