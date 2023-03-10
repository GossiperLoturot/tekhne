public class Context
{
    public static readonly TileContext tile;

    static Context()
    {
        tile = new TileContext(new GenerationRules());
    }
}