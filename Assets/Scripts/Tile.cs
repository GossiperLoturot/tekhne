using System;
using UnityEngine;

public interface ITile : IEquatable<ITile>
{
    public Vector3Int pos { get; }
    public string resourceName { get; }

    public void OnAfterAdd();

    public void OnBeforeRemove();
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

    public bool Equals(ITile other)
    {
        return pos == other.pos && resourceName == other.resourceName;
    }

    public void OnAfterAdd() { }

    public void OnBeforeRemove() { }
}