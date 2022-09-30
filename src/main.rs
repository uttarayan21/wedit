use anyhow::Result;
pub use types::World;

pub mod parser;
pub mod traits;
pub mod types;

impl types::World {
    pub fn from_slice(input: impl AsRef<[u8]>) -> Result<Self> {
        let input = input.as_ref();
        Self::parse(input)
            .map_err(|e| anyhow::anyhow!("Error parsing World: {:?}", e))
            .map(|(_, wld)| wld)
    }
}
pub fn main() -> Result<()> {
    let files: Vec<String> = std::env::args().skip(1).collect();
    for file in files {
        let world = World::from_slice(std::fs::read(file)?.as_slice())?;
        println!("{:#?}", world);
    }
    Ok(())
}
