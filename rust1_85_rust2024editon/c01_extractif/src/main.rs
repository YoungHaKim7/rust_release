fn main() {
    #![feature(extract_if)]
    let mut v = vec![0, 1, 2];
    let iter: std::vec::ExtractIf<'_, _, _> = v.extract_if(|x| *x % 2 == 0);

    println!("iter : {:?}", iter);
}
