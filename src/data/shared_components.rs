///This a marker that says the entity needs initiating
#[derive(Debug, Default, Clone, Copy)]
pub struct Uninitiated;
///This is a marker to help us know which [TileSetBundle](TileSetBundle) is currently viewed
#[derive(Debug, Default, Clone, Copy)]
pub struct CurrentlySelected;
