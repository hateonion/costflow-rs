use costflow_rs::{Constructor, CostFlowParser, Rule, Transaction};
use pest::Parser;
fn main() {
    let result = CostFlowParser::parse(Rule::transaction, "* 红包 #my-tag 66.66 工资卡 > 发红包");

    match result {
        Ok(mut ast) => {
            let transaction_ast = ast.next().unwrap().into_inner();

            let mut transaction = Transaction::new();

            for node in transaction_ast {
                match node.as_rule() {
                    Rule::narration => transaction.narration = node.as_str(),
                    Rule::complete_transaction => todo!(),
                    Rule::incomplete_transaction => todo!(),
                    Rule::year => todo!(),
                    Rule::month => todo!(),
                    Rule::day => todo!(),
                    Rule::date => transaction.date = node.as_str(),
                    Rule::flag => transaction.flag = node.as_str(),
                    Rule::content => todo!(),
                    Rule::tag => transaction.tag = node.as_str(),
                    Rule::link => transaction.link = node.as_str(),
                    Rule::amount => transaction.amount = node.as_str().parse::<f32>().unwrap(),
                    Rule::commodity => todo!(),
                    Rule::from_account => transaction.from_account = node.as_str(),
                    Rule::to_account => transaction.to_account = node.as_str(),
                    Rule::transaction => todo!(),
                    Rule::WHITESPACE => todo!(),
                }
            }
            println!("{:?}", transaction);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
