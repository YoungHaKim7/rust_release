// Parallelizing with Rayon
fn make_thumbnails(images: &[Image]) -> Vec<Image> {
    images
        .par_iter()
        .map(|image| image.make_thumbnail())
        .collect()
}

fn main() {
    println!("Hello, world!");
}
