use csv::Reader;
use std::error::Error;

pub fn load_dataset(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let (mut imdb_ratings, mut meta_scores) = (Vec::new(), Vec::new());

    for result in rdr.records() {
        let record = result?;
        if let (Ok(imdb), Ok(meta)) = (
            record.get(6).unwrap_or("").parse::<f64>(),
            record.get(8).unwrap_or("").parse::<f64>(),
        ) {
            imdb_ratings.push(imdb);
            meta_scores.push(meta);
        }
    }
    if imdb_ratings.is_empty() || imdb_ratings.len() != meta_scores.len() {
        return Err("Invalid dataset".into());
    }
    Ok((imdb_ratings, meta_scores))
}
