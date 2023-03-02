use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .compile(
            &["proto/health/v1/health.proto"],
            &["proto/"], // specify the root location to search proto dependencies
        )
        .unwrap();
    Ok(())
}
