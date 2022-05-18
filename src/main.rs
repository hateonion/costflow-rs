use std::fs::File;

use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use costflow_rs::{CostFlowParser, Rule, Transaction};
use pest::Parser;
fn main() {
    let result = CostFlowParser::parse(
        Rule::transaction,
        "20220501 红包 #my-tag 66.66 工资卡 > 发红包",
    );

    match result {
        Ok(mut ast) => {
            let transaction_ast = ast.next().unwrap().into_inner();

            let mut transaction = Transaction::new();

            let config_file = File::open("./config.json").expect("file open failed");

            let config_json: serde_json::Value =
                serde_json::from_reader(config_file).expect("config parse failed");

            let mut year: Option<&str>;
            let mut month: Option<&str>;
            let mut day: Option<&str>;

            println!(r#"{:?}"#, config_json);

            // Parse the string of data into serde_json::Value.
            // Access parts of the data by indexing with square brackets.
            for node in transaction_ast {
                match node.as_rule() {
                    Rule::narration => transaction.narration = node.as_str(),
                    // Rule::complete_transaction => todo!(),
                    // Rule::incomplete_transaction => todo!(),
                    // Rule::year => todo!(),
                    // Rule::month => todo!(),
                    // Rule::day => todo!(),
                    Rule::date => {
                        let inner_nodes = node.into_inner();
                        for _node in inner_nodes {
                            match _node.as_rule() {
                                Rule::month => month = Some(_node.as_str()),
                                Rule::day => day = Some(_node.as_str()),
                                Rule::year => year = Some(_node.as_str()),
                                _ => todo!(),
                            }
                        }
                    }
                    Rule::flag => transaction.flag = node.as_str(),
                    // Rule::content => todo!(),
                    Rule::tag => transaction.tag = node.as_str(),
                    Rule::link => transaction.link = node.as_str(),
                    Rule::amount => transaction.amount = node.as_str().parse::<f32>().unwrap(),
                    // Rule::commodity => todo!(),
                    Rule::from_account => transaction.from_account = node.as_str(),
                    Rule::to_account => transaction.to_account = node.as_str(),
                    // Rule::transaction => todo!(),
                    // Rule::WHITESPACE => todo!(),
                    _ => todo!(),
                }
            }
            let now = Utc::now()
                .with_timezone(&Shanghai)
                .format("%Y-%m-%d")
                .to_string();
            let date = if transaction.date.len() != 0 {
                transaction.date
            } else {
                now.as_str()
            };
            println!("{:?}", date);
            println!("{:?}", transaction);
        }
        Err(err) => {
            println!("Error{:?}", err);
        }
    }
}
