#[macro_export]
macro_rules! log {
    ("g",$string: expr) => {   print!("{}",        Color::Green.paint(format!("[✓] {}",$string))); };
   ("gl",$string: expr) => { println!("{}", Color::Green.bold().paint(format!("[✓] {}",$string))); };
    ("y",$string: expr) => { println!("{}",       Color::Yellow.paint(format!("[✱] {}",$string))); };
    ("c",$string: expr) => { println!("{}",         Color::Cyan.paint(format!("[◔] {}",$string))); };
    ("m",$string: expr) => { println!("{}",    Color::Fixed(13).paint(format!("[◎] {}",$string))); };
    ("r",$string: expr) => { println!("{}",          Color::Red.paint(format!("[✖] {}",$string))); };
    // ("g",$string: expr,$feeds: expr) => { println!("{}",  Color::Green.paint(format!(format!("[✓] {}",$string),$feeds))); };
    // ("y",$string: expr,$feeds: expr) => { println!("{}", Color::Yellow.paint(format!(format!("[◔] {}",$string),$feeds))); };
    // ("r",$string: expr,$feeds: expr) => { println!("{}",    Color::Red.paint(format!(format!("[x] {}",$string),$feeds))); };
}