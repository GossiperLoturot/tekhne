using System;
using System.Collections.Generic;
using UnityEngine;

public class TileService
{
    private const int GROUP_SIZE = 16;

    private readonly Dictionary<Vector3Int, Dictionary<Vector3Int, ITile>> tiles;

    private BoundsInt? updateGroupBounds;
    private Queue<ITileCommand> updateCommands;

    public TileService()
    {
        tiles = new();
        updateCommands = new();
    }

    public void AddTile(ITile tile)
    {
        var group = Vector3Int.FloorToInt((Vector3)tile.pos / GROUP_SIZE);

        if (tiles.ContainsKey(group) && tiles[group].ContainsKey(tile.pos))
        {
            throw new Exception("tile is already existed");
        }

        if (!tiles.ContainsKey(group))
        {
            tiles.Add(group, new());
        }
        tiles[group].Add(tile.pos, tile);
        tile.OnAfterAdd();

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds.InclusiveContains(group))
            {
                updateCommands.Enqueue(new AddTileCommand(tile));
            }
        }
    }

    public void UpdateTile(ITile tile)
    {
        var group = Vector3Int.FloorToInt((Vector3)tile.pos / GROUP_SIZE);

        if (!tiles.ContainsKey(group) || !tiles[group].ContainsKey(tile.pos))
        {
            throw new Exception("tile is not founded");
        }

        var prev = tiles[group][tile.pos];
        prev.OnBeforeRemove();
        tiles[group][tile.pos] = tile;
        tile.OnAfterAdd();

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds.InclusiveContains(group))
            {
                updateCommands.Enqueue(new RemoveTileCommand(tile.pos));
                updateCommands.Enqueue(new AddTileCommand(tile));
            }
        }
    }

    public void RemoveTile(Vector3Int pos)
    {
        var group = Vector3Int.FloorToInt((Vector3)pos / GROUP_SIZE);

        if (!tiles.ContainsKey(group) || !tiles[group].ContainsKey(pos))
        {
            throw new Exception("tile is not founded");
        }

        var prev = tiles[group][pos];
        prev.OnBeforeRemove();
        tiles[group].Remove(pos);
        if (tiles[group].Count == 0)
        {
            tiles.Remove(group);
        }

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds.InclusiveContains(group))
            {
                updateCommands.Enqueue(new RemoveTileCommand(pos));
            }
        }
    }

    public bool ContainsTile(Vector3Int pos)
    {
        var group = Vector3Int.FloorToInt((Vector3)pos / GROUP_SIZE);

        return tiles.ContainsKey(group) && tiles[group].ContainsKey(pos);
    }

    public ITile GetTile(Vector3Int pos)
    {
        var group = Vector3Int.FloorToInt((Vector3)pos / GROUP_SIZE);

        if (!tiles.ContainsKey(group) || !tiles[group].ContainsKey(pos))
        {
            throw new Exception("tile is not founded");
        }

        return tiles[group][pos];
    }

    public void SetUpdateBounds(BoundsInt bounds)
    {
        var groupBounds = new BoundsInt();
        var min = Vector3Int.FloorToInt((Vector3)bounds.min / GROUP_SIZE);
        var max = Vector3Int.FloorToInt((Vector3)bounds.max / GROUP_SIZE);
        groupBounds.SetMinMax(min, max);

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (groupBounds != updateGroupBounds)
            {
                for (var x = updateGroupBounds.xMin; x <= updateGroupBounds.xMax; x++)
                {
                    for (var y = updateGroupBounds.yMin; y <= updateGroupBounds.yMax; y++)
                    {
                        for (var z = updateGroupBounds.zMin; z <= updateGroupBounds.zMax; z++)
                        {
                            var group = new Vector3Int(x, y, z);
                            if (!groupBounds.InclusiveContains(group))
                            {
                                if (tiles.ContainsKey(group))
                                {
                                    foreach (var (pos, tile) in tiles[group])
                                    {
                                        updateCommands.Enqueue(new RemoveTileCommand(pos));
                                    }
                                }
                            }
                        }
                    }
                }

                for (var x = groupBounds.xMin; x <= groupBounds.xMax; x++)
                {
                    for (var y = groupBounds.yMin; y <= groupBounds.yMax; y++)
                    {
                        for (var z = groupBounds.zMin; z <= groupBounds.zMax; z++)
                        {
                            var group = new Vector3Int(x, y, z);
                            if (!updateGroupBounds.InclusiveContains(group))
                            {
                                if (tiles.ContainsKey(group))
                                {
                                    foreach (var (pos, tile) in tiles[group])
                                    {
                                        updateCommands.Enqueue(new AddTileCommand(tile));
                                    }
                                }
                            }
                        }
                    }
                }

                this.updateGroupBounds = groupBounds;
            }
        }
        else
        {
            for (var x = groupBounds.xMin; x <= groupBounds.xMax; x++)
            {
                for (var y = groupBounds.yMin; y <= groupBounds.yMax; y++)
                {
                    for (var z = groupBounds.zMin; z <= groupBounds.zMax; z++)
                    {
                        var group = new Vector3Int(x, y, z);
                        if (tiles.ContainsKey(group))
                        {
                            foreach (var (pos, tile) in tiles[group])
                            {
                                updateCommands.Enqueue(new AddTileCommand(tile));
                            }
                        }
                    }
                }
            }

            this.updateGroupBounds = groupBounds;
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
