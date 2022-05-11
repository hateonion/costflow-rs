extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "costflow.pest"]
pub struct CostFlowParser;


#[derive(Debug)]
pub struct Transaction<'a> {
    pub date: &'a str,
    pub amount: f32,
    pub flag: &'a str,
    pub tag: &'a str,
    pub link: &'a str,
    pub from_account: &'a str,
    pub to_account: &'a str,
    pub narration: &'a str,
}

pub trait Constructor {
    fn new() -> Self;
}
impl Constructor for Transaction<'static>  {
    fn new() -> Transaction<'static> {
        Transaction { date: "", amount: 0.0, flag: "", tag: "", link: "", from_account: "", to_account: "", narration: "" }
    }
}
