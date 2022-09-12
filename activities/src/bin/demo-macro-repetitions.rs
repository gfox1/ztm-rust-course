macro_rules! myvec {
    (
        $( $element:expr ),+
        $(,)?
    ) => {{
        let mut v = Vec::new();
        $(
        v.push($element);
        )+
        v
    }};
}

fn main() {
    let v = myvec![1,2,3,4];
}
