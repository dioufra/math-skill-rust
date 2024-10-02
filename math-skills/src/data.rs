use std::{
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub fn read_data_file(filename: &String) -> Result<Vec<f64>, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("src/{}", filename));

    let file = std::fs::File::open(path)?;

    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => match line.parse::<f64>() {
                Ok(value) => {
                    data.push(value);
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(data)
}
