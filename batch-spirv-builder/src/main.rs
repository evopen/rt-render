use spirv_builder::{SpirvBuilder, MemoryModel};
use std::error::Error;
use std::path::{PathBuf, Path};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let shader_dir = project_dir.join("../shader");
    std::fs::read_dir(shader_dir).unwrap().par_bridge().into_par_iter().map(|e| { e.unwrap().path() }).for_each(|p| {
        let spv_path = SpirvBuilder::new(p)
            .spirv_version(1, 3)
            .memory_model(MemoryModel::Vulkan)
            .build().unwrap();
        println!("{}", spv_path.to_str().unwrap());
    });


    Ok(())
}
