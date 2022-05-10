extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;


#[derive(Parser)]
#[grammar = "costflow.pest"]
pub struct CostFlowParser;

fn main() {
    let result = CostFlowParser::parse(Rule::transaction, "红包 #my-tag 66.66 工资卡 > 发红包");
    match result {
        Ok(transactions) => {
            println!("{:?}", transactions);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
