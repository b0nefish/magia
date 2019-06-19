use pest;
use pest_derive::Parser;
use pest::Parser;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const CPU_CONFIG: &str = include_str!("src/gen_config.txt");

#[derive(Parser)]
#[grammar = "cpu_config.pest"]
struct CpuGenerator;

impl CpuGenerator {
    pub fn process() {
        let _chunks = CpuGenerator::parse(Rule::chunk, CPU_CONFIG)
            .unwrap_or_else(|e| panic!("CpuGenerator: {}", e));

        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("hello.rs");

        let data = format!("{}", _chunks);

        let mut f = File::create(&dest_path).unwrap();
        writeln!(f, "{}", data).unwrap();
    }
}
fn main() {
    CpuGenerator::process();
}
