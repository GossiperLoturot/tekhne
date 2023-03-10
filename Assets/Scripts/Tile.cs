public interface ITile
{
    public int x { get; }
    public int y { get; }
    public int layer { get; }
    public string resourceName { get; }

    public void OnSet();
    public void OnRemove();
}

public class Tile : ITile
{
    public int x { get; private set; }
    public int y { get; private set; }
    public int layer { get; private set; }
    public string resourceName { get; }

    public Tile(int x, int y, int layer, string resourceName)
    {
        this.x = x;
        this.y = y;
        this.layer = layer;
        this.resourceName = resourceName;
    }

    public void OnSet() { }
    public void OnRemove() { }
}