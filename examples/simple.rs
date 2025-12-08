pub fn main() {}

spoke::test! {
    $"a string" {
        let mut s = String::new();
        $"begins" {
            $"empty" s.is_empty();
            $"with zero size" 0 $eq s.len();
        }
    }
}
