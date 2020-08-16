use structopt::StructOpt;
use std::fmt::{self, Display, Formatter};

#[derive(StructOpt)]
#[structopt(name = "app")]
pub struct AppArgs {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// add operation

    //打印偶数
    #[structopt(name = "even")]
    Even(Elements),
    // 打印奇数
    #[structopt(name = "odd")]
    Odd(Elements),

}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<i32>,
}
//实现display
impl Display for Elements {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#?}]", self.elements)
    }
}

fn main() {
    let opt = AppArgs::from_args();
    let opt2 = opt;//所有权转移
    match opt2.command {
        
        Command::Even(e) =>{
            println!("Operants: {}", e);
            print!("Result: ");
            //迭代器访问
            for i in &e.elements {
                if i%2 == 0{
                    print!(" {} ", i);
                }
            }
        },
        Command::Odd(e) =>{
            println!("Operants: {}", e);
            print!("Result: ");
            for i in &e.elements {
                if i%2 == 1{
                    print!(" {} ", i);
                }
            }
        },

    }
}