use reqwest;
use std::fs;
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "vote/index.php";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    println!("Hasil:\n{}", body);
    Ok(())
}
fn scan_file(file_path: &str) -> bool {
    if file_path.ends_with(".virus") {
        println!("Found virus in file: {}", file_path);
        return true;
    }
    false
}

fn main() {
    let files_to_scan = vec![
        "ping.bat",
    ];

    for file_path in files_to_scan {
        if scan_file(file_path) {
            println!("Q: {}", file_path);

            // Simulate quarantining the file by moving it to a different directory
            let quarantine_dir = "q";
            if let Err(err) = fs::create_dir_all(quarantine_dir) {
                eprintln!("AAAA: {}", err);
                return;
            }

            let new_path = format!("{}/{}", quarantine_dir, file_path);
            if let Err(err) = fs::rename(file_path, &new_path) {
                eprintln!("AAAA: {}", err);
            }
        } else {
            println!("OWO {}.", file_path);
        }
    }
}
