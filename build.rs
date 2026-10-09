use std::{env, fs, io::Write, path::PathBuf};

use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenOptions, CommentOptions};
use oxc_minifier::{CompressOptions, Minifier, MinifierOptions};
use oxc_parser::Parser;
use oxc_span::SourceType;

const PUBLIC_ASSETS: &[&str] = &[
    "https://unpkg.com/maplibre-gl@6.13.0/dist/maplibre-gl.mjs",
    "https://unpkg.com/maplibre-gl@6.13.0/dist/maplibre-gl-worker.mjs",
    "https://unpkg.com/pmtiles@4.5.0/dist/pmtiles.js",
];
fn main() {
    println!("cargo:rerun-if-changed=styles.css");
    println!("cargo:rerun-if-changed=static/script.js");
    println!("cargo:rerun-if-changed=src");
    
    minify_script().expect("failed to minify static script");
    download_public_assets().expect("failed to download static public assets");
    topcoat::tailwind::BuildConfig::new().input("styles.css").render().unwrap();
}

fn minify_script() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let source_path = manifest_dir.join("static/script.js");
    let public_dir = manifest_dir.join("static/public");
    let output_path = public_dir.join("script.js");

    fs::create_dir_all(&public_dir)?;

    let source = fs::read_to_string(&source_path)?;
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &source, SourceType::mjs()).parse();
    if !parsed.diagnostics.is_empty() {
        return Err(format!("failed to parse {source_path:?}: {:?}", parsed.diagnostics).into());
    }

    let mut program = parsed.program;
    let minified = Minifier::new(MinifierOptions {
        mangle: Some(Default::default()), // Replace with None to disable name mangling
        mangle_properties: None,
        compress: Some(CompressOptions::smallest()),
    })
    .minify(&allocator, &mut program);
    let output = Codegen::new()
        .with_options(CodegenOptions {
            minify: true,
            comments: CommentOptions::disabled(),
            ..CodegenOptions::default()
        })
        .with_scoping(minified.scoping)
        .build(&program)
        .code;

    fs::write(&output_path, output)?;
    compress_asset(&output_path, "script.js")?;

    Ok(())
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
    if compression_is_stale(asset_path, &gzip_path)? {
        println!("cargo:warning=Compressing {filename} with maximum gzip compression");
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(&fs::read(asset_path)?)?;
        fs::write(gzip_path, encoder.finish()?)?;
    }

    let brotli_path = asset_path.with_file_name(format!("{filename}.br"));
    if compression_is_stale(asset_path, &brotli_path)? {
        println!("cargo:warning=Compressing {filename} with maximum Brotli compression");
        let mut encoder = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
        encoder.write_all(&fs::read(asset_path)?)?;
        encoder.flush()?;
        fs::write(brotli_path, encoder.into_inner())?;
    }

    Ok(())
}

fn compression_is_stale(
    asset_path: &PathBuf,
    compressed_path: &PathBuf,
) -> Result<bool, Box<dyn std::error::Error>> {
    let asset_modified = fs::metadata(asset_path)?.modified()?;
    let compressed_modified = match fs::metadata(compressed_path) {
        Ok(metadata) => metadata.modified()?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(true),
        Err(error) => return Err(error.into()),
    };

    Ok(compressed_modified < asset_modified)
}