public class Context
{
    public static readonly TileContext tile;
    public static readonly EntityContext entity;

    static Context()
    {
        tile = new TileContext(new GenerationRules());
        entity = new EntityContext();
    }
}