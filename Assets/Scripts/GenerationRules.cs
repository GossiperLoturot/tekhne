public class GenerationRules : TileContext.IGenerationRules
{
    public ITile GenerateTile(int x, int y, int layer)
    {
        if (layer == 0)
        {
            return new Tile(x, y, layer, "SurfaceStone");
        }
        else
        {
            return null;
        }
    }
}