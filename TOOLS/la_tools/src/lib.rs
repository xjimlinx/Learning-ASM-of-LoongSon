pub mod lart_utils {
    pub fn decimal_to_binary(mut num: u64) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut binary = Vec::new();
        while num > 0 {
            binary.push((num % 2).to_string());
            num /= 2;
        }
        binary.reverse();
        binary.join("")
    }

    pub fn group_binary(binary_str: &str) -> String {
        let mut grouped = String::new();
        let len = binary_str.len();

        for (i, c) in binary_str.chars().enumerate() {
            grouped.push(c);
            if (len - i - 1) % 4 == 0 && i != len - 1 {
                grouped.push(',');
            }
        }

        grouped
    }

    pub fn format_binary_table(binary_str: &str) -> Vec<String> {
        let len = binary_str.len();
        let mut lines = Vec::new();

        // 生成顶线
        let mut top_line = String::from("╭───");
        for _ in 1..len {
            top_line.push_str("┬───");
        }
        top_line.push('╮');
        lines.push(top_line);

        // 生成索引行 (从右向左编号)
        let mut index_line = String::from("│");
        for i in 0..len {
            index_line.push_str(&format!("{:^3}│", len - i));
        }
        lines.push(index_line);

        // 生成中线
        let mut mid_line = String::from("├───");
        for _ in 1..len {
            mid_line.push_str("┼───");
        }
        mid_line.push('┤');
        lines.push(mid_line);

        // 生成数值行
        let mut value_line = String::from("│");
        for c in binary_str.chars() {
            value_line.push_str(&format!(" \x1b[1m{}\x1b[0m │", c)); // 加粗显示二进制值
        }
        lines.push(value_line);

        // 生成底线
        let mut bottom_line = String::from("╰───");
        for _ in 1..len {
            bottom_line.push_str("┴───");
        }
        bottom_line.push('╯');
        lines.push(bottom_line);

        lines
    }
}
