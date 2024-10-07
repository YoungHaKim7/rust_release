async fn batch_job(db: &Database) {
    let rows = run_query(db, FIND_WORK_QUERY).await;
    let resutls = stream::iter(rows).map(|row| process_row(row)).buffered(5);
    while let Some(result) = results.iter().next().await {
        upload_result(result).await;
    }
}

fn main() {
    println!("Hello, world!");
}
