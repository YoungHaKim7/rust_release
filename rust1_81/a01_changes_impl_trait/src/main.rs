#[derive(Default, Debug)]
struct Datum {
    value: i32,
}

impl Datum {
    fn process(&self) -> ProcessedDatum {
        ProcessedDatum {
            value: self.value + 1,
        }
    }
}

#[derive(Debug)]
struct ProcessedDatum {
    value: i32,
}

fn process_data<'d>(data: &'d [Datum]) -> impl Iterator<Item = ProcessedDatum> + 'd {
    data.iter().map(|datum| datum.process())
}

fn main() {
    let mut data: Vec<Datum> = vec![Datum::default()];

    // Process data before mutating the original vector
    let processed_data: Vec<ProcessedDatum> = process_data(&data).collect();

    // Now it's safe to mutate the data
    data.push(Datum::default()); // This is now allowed

    // Optionally, do something with `processed_data`
    for item in processed_data {
        println!("{:?}", item);
    }
}
