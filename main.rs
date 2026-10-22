use serde::Deserialize;
use std::error::Error;
use std::fs::File;

// 1. Define the record object for your periphyton dataset
#[derive(Debug, Deserialize)]
struct PeriphytonRecord {
    #[serde(alias = "Identification", alias = "identification", alias = "IDENTIFICATION")]
    identification: String,

    #[serde(alias = "Lake identification", alias = "lake identification", alias = "LAKE IDENTIFICATION")]
    lake_identification: String,

    #[serde(alias = "Species code", alias = "species code", alias = "SPECIES CODE")]
    species_code: String,

    #[serde(alias = "Species", alias = "species", alias = "SPECIES")]
    species: String,

    #[serde(alias = "Percent abundance", alias = "percent abundance", alias = "PERCENT ABUNDANCE")]
    percent_abundance: f64,
}

fn read_vuntut_dataset(file_path: &str) -> Result<(), Box<dyn Error>> {
    // 2. Open the specific dataset file
    let file = File::open(file_path)?;
    
    // 3. Create the CSV reader
    let mut reader = csv::Reader::from_reader(file);

    println!("--- Processing Vuntut NP Periphyton Records ---");

    // 4. Deserialize and print every record object
    for (index, result) in reader.deserialize().enumerate() {
        match result {
            Ok(record) => {
                // Type-safe printing using the struct
                let r: PeriphytonRecord = record;
                println!("Record #{}: {:?}", index + 1, r);
            }
            Err(err) => {
                // Gracefully catch formatting errors (like a missing float or corrupt text line)
                eprintln!("Row {} failed to parse: {}", index + 1, err);
            }
        }
    }

    Ok(())
}

fn main() {
    // The exact name of your periphyton dataset file
    let path = "vuntut_np_lake_periphyton_community_structure_2012_2016_data.csv"; 
    
    if let Err(err) = read_vuntut_dataset(path) {
        eprintln!("Error opening or reading file: {}", err);
        std::process::exit(1);
    }
}
