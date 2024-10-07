// fn make_thumbnails(images: &[Image]) -> Vec<Image> {
//     let mut i = 0;
//     let l = images.len();
//     let mut output = Vec::with_capacity(l);
//     while i < l {
//         output.push(images[i].make_thumbnail());
// --unsafe- > output.push(unsafe{imgaes.get_unchecked(i).make_thumbnail()});
//         i += 1;
//     }
//     output
// }

fn make_thumbnails(images: &[Image]) -> Vec<Image> {
    images.iter().map(|image| image.make_thumbnail()).collect()
}

fn main() {
    println!("Hello, world!");
}
