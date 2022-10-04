use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/example3.proto"], &["src/"])?;
    prost_build::compile_protos(&["src/example2.proto"], &["src/"])?;
    Ok(())
}
