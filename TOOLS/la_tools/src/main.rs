use std::io;
pub use lart::lart_utils;
fn main() {
    println!("🔢 十进制转二进制表格工具");
    loop {
        println!("请输入一个正整数:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");

        match input.trim().parse() {
            Ok(num) => {
                let binary_str = lart_utils::decimal_to_binary(num);
                let group_str = lart_utils::group_binary(&binary_str);
                println!("\n🔧 格式化结果 (0d{} → 0b{})", num, group_str);
                for line in lart_utils::format_binary_table(&binary_str) {
                    println!("{}", line);
                }
                continue;
            }
            Err(_) => {
                println!("请输入有效的正整数，再试一次:");
                continue;
            }
        }
    }
}
