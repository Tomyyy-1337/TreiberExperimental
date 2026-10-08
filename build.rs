use std::{env, fs, io::Write, path::PathBuf};

const PUBLIC_ASSETS: &[&str] = &[
    "https://unpkg.com/maplibre-gl@6.13.0/dist/maplibre-gl.mjs",
    "https://unpkg.com/maplibre-gl@6.13.0/dist/maplibre-gl-worker.mjs",
    "https://unpkg.com/pmtiles@4.5.0/dist/pmtiles.js",
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    download_public_assets().expect("failed to download static public assets");
    topcoat::tailwind::BuildConfig::new().input("styles.css").render().unwrap();
}

fn download_public_assets() -> Result<(), Box<dyn std::error::Error>> {
    let public_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("static/public");
    fs::create_dir_all(&public_dir)?;

    for url in PUBLIC_ASSETS {
        let parsed_url = reqwest::Url::parse(url)?;
        let filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .filter(|filename| !filename.is_empty())
            .ok_or_else(|| format!("asset URL has no filename: {url}"))?;
        let asset_path = public_dir.join(filename);
        if !asset_path.exists() {
            println!("cargo:warning=Downloading {filename} from {url}");
            let response = reqwest::blocking::get(*url)?.error_for_status()?;
            fs::write(&asset_path, response.bytes()?)?;
        }

        compress_asset(&asset_path, filename)?;
    }

    Ok(())
}

fn compress_asset(asset_path: &PathBuf, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let gzip_path = asset_path.with_file_name(format!("{filename}.gz"));
    if gzip_path.exists() {
        return Ok(());
    }

    println!("cargo:warning=Compressing {filename} with maximum gzip compression");
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(&fs::read(asset_path)?)?;
    fs::write(gzip_path, encoder.finish()?)?;

    Ok(())
}