#[macro_export]
// Macro to create constant arrays
macro_rules! create_array {
    ($constructor:expr; $count:expr) => {
        {
            use seq_macro::seq;
            seq!(N in 0..$count {
                [
                    #( $constructor, )*
                ]
            })
        }
    };
}