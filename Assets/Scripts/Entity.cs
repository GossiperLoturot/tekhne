public interface IEntity
{
    public float x { get; }
    public float y { get; }
    public float layer { get; }
    public string resourceName { get; }
}

public class Entity : IEntity
{
    public float x { get; private set; }
    public float y { get; private set; }
    public float layer { get; private set; }
    public string resourceName { get; private set; }

    public Entity(float x, float y, float layer, string resourceName)
    {
        this.x = x;
        this.y = y;
        this.layer = layer;
        this.resourceName = resourceName;
    }
}