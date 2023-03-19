using UnityEngine;

public interface ITile
{
    public Vector3Int pos { get; }
    public string resourceName { get; }

    public void OnAdd();

    public void OnRemove();
}

public class Tile : ITile
{
    public Vector3Int pos { get; private set; }
    public string resourceName { get; }

    public Tile(Vector3Int pos, string resourceName)
    {
        this.pos = pos;
        this.resourceName = resourceName;
    }

    public void OnAdd() { }

    public void OnRemove() { }
}