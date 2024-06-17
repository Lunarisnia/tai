#[macro_export]
macro_rules! greet {
    ($($name:tt),+) => {
        $({
            $crate::println!("Hi {}!", $name)
        })+
    };
}