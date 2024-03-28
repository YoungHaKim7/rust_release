use std::sync::atomic::AtomicIsize;

// Parallelizing with Rayon
fn make_thumbnails(images: &[Image]) -> Vec<Image> {
    let counter = AtomicIsize::new();
    let vec = images
        .par_iter()
        .map(|image| {
            counter.fetch_add(1, Ordering::SeqCst);
            image.make_thumbnail()
        })
        .collect();
    log(counter.load(Ordering::SeqCst));
    vec
}

fn main() {
    println!("Hello, world!");
}
