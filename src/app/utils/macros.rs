#[macro_export]
macro_rules! info_message {
    () => {
        let message = "[info]";
        println!("{}", message.bold().green());
    };
    ($($arg:tt)*) => {
        let message ="[info]";
        print!("{} ",message.bold().green());
        println!($($arg)*);
    }
}
