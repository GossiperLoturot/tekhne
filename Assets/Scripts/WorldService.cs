public class WorldService
{
    public static readonly TileService tile;
    public static readonly EntityService entity;
    public static readonly GenerationService generation;

    static WorldService()
    {
        tile = new TileService();
        entity = new EntityService();
        generation = new GenerationService(tile);
    }
}