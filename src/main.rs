#![allow(dead_code,non_camel_case_types)]
use pest::{Parser,pratt_parser::{self, PrattParser}};
use pest_derive::Parser;
#[derive(Parser)]
#[grammar="escalator.pest"]
pub struct escalator;
pub enum opr{add,sub,mul,div}
pub struct vm{
    ds:Vec<i32>,
    is:Vec<opr>,
}
pub fn prs(s:&str,v:&mut vm){
    let pp:PrattParser<> = PrattParser::new();
    let f = escalator::parse(Rule::prg,s).expect("tamere").next().unwrap();
    for x in f.into_inner(){ 
        match x.as_rule(){
            Rule::num=>v.ds.push(x.as_str().parse::<i32>().unwrap()),
            Rule::add=>v.is.push(opr::add),
            _=>(),
        }
    }
}
fn main() {
    println!("Hello, world!");
}
