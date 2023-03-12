using System;
using System.Collections.Generic;

public class TileService
{
    private readonly Dictionary<(int, int, int), ITile> tiles;
    private readonly HashSet<(int, int, int)> initFlags;
    private readonly IGenerationRules genRules;

    public TileService(IGenerationRules genRules)
    {
        tiles = new();
        initFlags = new();
        this.genRules = genRules;
    }

    public void SetTile(ITile tile)
    {
        var (x, y, layer) = (tile.x, tile.y, tile.layer);

        GenerateTile(x, y, layer);

        if (tiles.ContainsKey((x, y, layer)))
        {
            tiles[(x, y, layer)] = tile;
        }
        else
        {
            tiles.Add((x, y, layer), tile);
        }

        tile.OnSet();
    }

    public ITile GetTile(int x, int y, int layer)
    {
        GenerateTile(x, y, layer);

        if (tiles.ContainsKey((x, y, layer)))
        {
            return tiles[(x, y, layer)];
        }
        else
        {
            return null;
        }
    }

    public void RemoveTile(int x, int y, int layer)
    {
        GenerateTile(x, y, layer);

        if (tiles.ContainsKey((x, y, layer)))
        {
            var tile = tiles[(x, y, layer)];
            tile.OnRemove();

            tiles.Remove((x, y, layer));
        }
    }

    private void GenerateTile(int x, int y, int layer)
    {
        if (!initFlags.Contains((x, y, layer)))
        {
            if (tiles.ContainsKey((x, y, layer)))
            {
                throw new Exception("invalid tiles");
            }

            var tile = genRules.GenerateTile(x, y, layer);

            if (tile != null)
            {
                if (tile.x != x || tile.y != y || tile.layer != tile.layer)
                {
                    throw new Exception("invalid generate rules");
                }

                tiles.Add((x, y, layer), tile);

                tile.OnSet();
            }

            initFlags.Add((x, y, layer));
        }
    }

    public interface IGenerationRules
    {
        public ITile GenerateTile(int x, int y, int layer);
    }
}
