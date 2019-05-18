use pest;
use pest_derive::Parser;
use pest::Parser;

const CPU_CONFIG: &str = include_str!("src/gen_config.txt");

#[derive(Parser)]
#[grammar = "cpu_config.pest"]
struct CpuGenerator;

impl CpuGenerator {
    pub fn process() {
        let _chunks = CpuGenerator::parse(Rule::chunk, CPU_CONFIG)
            .unwrap_or_else(|e| panic!("CpuGenerator: {}", e));
    }
}
fn main() {
    CpuGenerator::process();
}
