#[macro_export]
macro_rules! log {
    ("g",$string: expr) => { println!("{}",   Color::Green.paint(format!("[✓] {}",$string))); };
    ("y",$string: expr) => { println!("{}",  Color::Yellow.paint(format!("[✱] {}",$string))); };
    ("c",$string: expr) => { println!("{}",    Color::Cyan.paint(format!("[◔] {}",$string))); };
    ("p",$string: expr) => { println!("{}",  Color::Purple.paint(format!("[◎] {}",$string))); };
    ("r",$string: expr) => { println!("{}",     Color::Red.paint(format!("[✖] {}",$string))); };
    ("g",$string: expr,$feeds: expr) => { println!("{}",  Color::Green.paint(format!(format!("[✓] {}",$string),$feeds))); };
    ("y",$string: expr,$feeds: expr) => { println!("{}", Color::Yellow.paint(format!(format!("[◔] {}",$string),$feeds))); };
    ("r",$string: expr,$feeds: expr) => { println!("{}",    Color::Red.paint(format!(format!("[x] {}",$string),$feeds))); };
}