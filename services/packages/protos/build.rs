fn main() -> Result<(), Box<dyn std::error::Error>> {
  std::fs::create_dir_all("src/generated")?;
  
  prost_build::Config::new()
      .out_dir("src/generated")
      .compile_protos(
          &["defs/events.proto"], 
          &["defs/"]
      )?;
  
  println!("cargo:rerun-if-changed=defs/");
  Ok(())
}