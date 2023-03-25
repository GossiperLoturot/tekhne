using System.Collections.Generic;
using UnityEngine;

public class TileService
{
    private readonly Dictionary<Vector3Int, ITile> tiles;

    private BoundsInt? updateBounds;
    private Queue<ITileCommand> updateCommands;

    public TileService()
    {
        tiles = new();
        updateCommands = new();
    }

    public void AddTile(ITile tile)
    {
        tiles.Add(tile.pos, tile);

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.InclusiveContains(tile.pos))
            {
                updateCommands.Enqueue(new AddTileCommand(tile));
            }
        }
    }

    public void UpdateTile(ITile tile)
    {
        var prev = tiles[tile.pos];

        tiles[tile.pos] = tile;

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.InclusiveContains(prev.pos))
            {
                updateCommands.Enqueue(new RemoveTileCommand(prev.pos));
            }

            if (updateBounds.InclusiveContains(tile.pos))
            {
                updateCommands.Enqueue(new AddTileCommand(tile));
            }
        }
    }

    public void RemoveTile(Vector3Int pos)
    {
        var tile = tiles[pos];

        tiles.Remove(tile.pos);

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.InclusiveContains(tile.pos))
            {
                updateCommands.Enqueue(new RemoveTileCommand(tile.pos));
            }
        }
    }

    public bool ContainsTile(Vector3Int pos)
    {
        return tiles.ContainsKey(pos);
    }

    public ITile GetTile(Vector3Int pos)
    {
        return tiles[pos];
    }

    public void SetUpdateBounds(BoundsInt bounds)
    {
        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (bounds != updateBounds)
            {
                for (var x = updateBounds.xMin; x <= updateBounds.xMax; x++)
                {
                    for (var y = updateBounds.yMin; y <= updateBounds.yMax; y++)
                    {
                        for (var z = updateBounds.zMin; z <= updateBounds.zMax; z++)
                        {
                            var pos = new Vector3Int(x, y, z);
                            if (!bounds.InclusiveContains(pos))
                            {
                                if (tiles.ContainsKey(pos))
                                {
                                    updateCommands.Enqueue(new RemoveTileCommand(pos));
                                }
                            }
                        }
                    }
                }

                for (var x = bounds.xMin; x <= bounds.xMax; x++)
                {
                    for (var y = bounds.yMin; y <= bounds.yMax; y++)
                    {
                        for (var z = bounds.zMin; z <= bounds.zMax; z++)
                        {
                            var pos = new Vector3Int(x, y, z);
                            if (!updateBounds.InclusiveContains(pos))
                            {
                                if (tiles.ContainsKey(pos))
                                {
                                    var tile = GetTile(pos);
                                    updateCommands.Enqueue(new AddTileCommand(tile));
                                }
                            }
                        }
                    }
                }

                this.updateBounds = bounds;
            }
        }
        else
        {
            for (var x = bounds.xMin; x <= bounds.xMax; x++)
            {
                for (var y = bounds.yMin; y <= bounds.yMax; y++)
                {
                    for (var z = bounds.zMin; z <= bounds.zMax; z++)
                    {
                        var pos = new Vector3Int(x, y, z);
                        if (tiles.ContainsKey(pos))
                        {
                            var tile = GetTile(pos);
                            updateCommands.Enqueue(new AddTileCommand(tile));
                        }
                    }
                }
            }

            this.updateBounds = bounds;
        }
    }

    public Queue<ITileCommand> GetUpdateCommands()
    {
        var cmds = updateCommands;
        updateCommands = new();
        return cmds;
    }

    public interface ITileCommand { }

    public class AddTileCommand : ITileCommand
    {
        public ITile tile { get; private set; }

        public AddTileCommand(ITile tile)
        {
            this.tile = tile;
        }
    }

    public class RemoveTileCommand : ITileCommand
    {
        public Vector3Int pos { get; private set; }

        public RemoveTileCommand(Vector3Int pos)
        {
            this.pos = pos;
        }
    }
}
