using System;
using System.Collections.Generic;
using UnityEngine;

public class TileService
{
    private readonly Dictionary<(int, int), Dictionary<int, ITile>> tiles;

    private BoundsInt? updateBounds;
    private Queue<ITileCommand> updateCommands;

    public TileService()
    {
        tiles = new();
        updateCommands = new();
    }

    public void AddTile(ITile tile)
    {
        var (xy, z) = ((tile.pos.x, tile.pos.y), tile.pos.z);

        if (tiles.ContainsKey(xy) && tiles[xy].ContainsKey(z))
        {
            throw new Exception("tile is already existed");
        }

        if (!tiles.ContainsKey(xy))
        {
            tiles.Add(xy, new());
        }
        tiles[xy].Add(z, tile);

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
        var (xy, z) = ((tile.pos.x, tile.pos.y), tile.pos.z);

        if (!tiles.ContainsKey(xy) || !tiles[xy].ContainsKey(z))
        {
            throw new Exception("tile is not founded");
        }

        tiles[xy][z] = tile;

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.InclusiveContains(tile.pos))
            {
                updateCommands.Enqueue(new RemoveTileCommand(tile.pos));
                updateCommands.Enqueue(new AddTileCommand(tile));
            }
        }
    }

    public void RemoveTile(Vector3Int pos)
    {
        var (xy, z) = ((pos.x, pos.y), pos.z);

        if (!tiles.ContainsKey(xy) || !tiles[xy].ContainsKey(z))
        {
            throw new Exception("tile is not founded");
        }

        tiles[xy].Remove(z);
        if (tiles[xy].Count == 0)
        {
            tiles.Remove(xy);
        }

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.InclusiveContains(pos))
            {
                updateCommands.Enqueue(new RemoveTileCommand(pos));
            }
        }
    }

    public bool ContainsTile(Vector3Int pos)
    {
        var (xy, z) = ((pos.x, pos.y), pos.z);

        return tiles.ContainsKey(xy) && tiles[xy].ContainsKey(z);
    }

    public ITile GetTile(Vector3Int pos)
    {
        var (xy, z) = ((pos.x, pos.y), pos.z);

        if (!tiles.ContainsKey(xy) || !tiles[xy].ContainsKey(z))
        {
            throw new Exception("tile is not founded");
        }

        return tiles[xy][z];
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
                        if (tiles.ContainsKey((x, y)))
                        {
                            foreach (var (z, tile) in tiles[(x, y)])
                            {
                                if (bounds.zMin <= z && z <= bounds.zMax)
                                {
                                    var pos = new Vector3Int(x, y, z);
                                    if (!bounds.InclusiveContains(pos))
                                    {
                                        updateCommands.Enqueue(new RemoveTileCommand(pos));
                                    }
                                }
                            }
                        }
                    }
                }

                for (var x = bounds.xMin; x <= bounds.xMax; x++)
                {
                    for (var y = bounds.yMin; y <= bounds.yMax; y++)
                    {
                        if (tiles.ContainsKey((x, y)))
                        {
                            foreach (var (z, tile) in tiles[(x, y)])
                            {
                                if (bounds.zMin <= z && z <= bounds.zMax)
                                {
                                    var pos = new Vector3Int(x, y, z);
                                    if (!updateBounds.InclusiveContains(pos))
                                    {
                                        updateCommands.Enqueue(new AddTileCommand(tile));
                                    }
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
                    if (tiles.ContainsKey((x, y)))
                    {
                        foreach (var (z, tile) in tiles[(x, y)])
                        {
                            if (bounds.zMin <= z && z <= bounds.zMax)
                            {
                                updateCommands.Enqueue(new AddTileCommand(tile));
                            }
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
