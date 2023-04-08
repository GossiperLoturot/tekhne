public class WorldService
{
    public static readonly TileService tile;
    public static readonly EntityService entity;
    public static readonly GenerationService generation;
    public static readonly UpdateService update;

    static WorldService()
    {
        tile = new TileService();
        entity = new EntityService();
        generation = new GenerationService();
        update = new UpdateService();
    }
}