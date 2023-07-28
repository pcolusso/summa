use summa_store::Store;
use summa_cognition::Engine;
use color_eyre::Result;

fn main() -> Result<()> {
    let store = Store::new()?;

    let engine = Engine::new()?;

    let res = engine.encode("Hello, world!")?;

    println!("{res:?}");

    Ok(())
}
