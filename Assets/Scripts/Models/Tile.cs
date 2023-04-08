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

public class TileHarvestable : Tile, IHarvestable
{
    public IItem item { get; private set; }

    public TileHarvestable(Vector3Int pos, string resourceName, IItem item) : base(pos, resourceName) 
    {
        this.item = item;
    }

    public void OnHarvest()
    {
        var id = Guid.NewGuid().ToString();
        var entity = new EntityPickable(id, pos, item);
        WorldService.current.entity.AddEntity(entity);
        WorldService.current.tile.RemoveTile(pos);
    }
}