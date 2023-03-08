namespace Models
{
    public enum TileKind
    {
        SurfaceGrass,
        SurfaceDirt,
        SurfaceStone,
        SurfaceWater,
        SurfaceWaterDeep,
    }

    public class Tile
    {
        public int x { get; private set; }
        public int y { get; private set; }
        public int layer { get; private set; }
        public TileKind kind { get; private set; }

        public Tile(int x, int y, int layer, TileKind kind)
        {
            this.x = x;
            this.y = y;
            this.layer = layer;
            this.kind = kind;
        }
    }
}
