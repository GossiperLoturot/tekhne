public class Service
{
    public static readonly TileService tile;
    public static readonly EntityService entity;

    static Service()
    {
        tile = new TileService(new GenerationRules());
        entity = new EntityService();
    }
}