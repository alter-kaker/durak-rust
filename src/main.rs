use durak_rust::prelude::*;

fn main() -> Result<(), GameError> {
    pollster::block_on(durak_rust::run())
}
