use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;

fn main() {
    let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva"];
    let (s, r) = bounded(1);

    let seek = |name, s, r| {
        select! {
            recv(r) -> peer => println!("{} received a messages from {}. ", name, peer.unwrap()),
            send(s, name) -> _ => {},
        }
    };
    
    thread::scope(|scope| {
        for name in people {
            let (s, r) = (s.clone(), r.clone());
            scope.spawn(move |_| seek(name, s, r));
        }
    })
    .unwrap();

    if let Ok(name) = r.try_recv() {
        println!("No one received {}'s message. ", name);
    }
}
