use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum UnitKind {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
    MixGrass,
    Dandelion,
    FallenBranch,
    FallenLeaves,
    MixPebbles,
    Player,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}
