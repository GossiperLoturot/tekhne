using System.Collections.Generic;
using UnityEngine;

public class GenerationService
{
    private readonly HashSet<Vector2Int> initFlags;

    private BoundsInt? updateBounds;

    public GenerationService()
    {
        initFlags = new();
    }

    public void SetUpdateBounds(BoundsInt bounds)
    {
        if (bounds != updateBounds)
        {
            for (var x = bounds.xMin; x <= bounds.xMax; x++)
            {
                for (var y = bounds.yMin; y <= bounds.yMax; y++)
                {
                    var pos = new Vector2Int(x, y);

                    if (!initFlags.Contains(pos))
                    {
                        // generation rules start

                        if (x == 0 && y == 0)
                        {
                            var id = System.Guid.NewGuid().ToString();
                            var entity = new EntityCircularMotion(id, Vector3.forward, "SurfaceSand", Vector3.forward);
                            WorldService.entity.AddEntity(entity);
                        }

                        var value = FBMNoise(x * 0.1f, y * 0.1f);
                        if (0.5f < value)
                        {
                            WorldService.tile.AddTile(new Tile(new Vector3Int(x, y, 0), "SurfaceStone"));

                            if (0.75f < Random.value)
                            {
                                WorldService.tile.AddTile(new Tile(new Vector3Int(x, y, 1), "ObjectPebbles"));
                            }
                        }
                        else
                        {
                            WorldService.tile.AddTile(new Tile(new Vector3Int(x, y, 0), "SurfaceGrass"));

                            var prob = Random.value;
                            if (0.5f <= prob && prob < 0.75f)
                            {
                                WorldService.tile.AddTile(new TileHarvestable(new Vector3Int(x, y, 1), "ObjectShortGrass", new Item("ItemGrass")));
                            }
                            else if (0.75f <= prob)
                            {
                                WorldService.tile.AddTile(new TileHarvestable(new Vector3Int(x, y, 1), "ObjectLongGrass", new Item("ItemGrass")));
                            }
                        }

                        // generation rules end

                        initFlags.Add(pos);
                    }
                }
            }

            updateBounds = bounds;
        }
    }

    private float FBMNoise(float x, float y)
    {
        const int ITERATIONS = 8;

        var value = 0.0f;
        var amplitude = 0.5f;
        var scale = 1.0f;

        for (var i = 0; i < ITERATIONS; i++)
        {
            value += Mathf.PerlinNoise(x * scale, y * scale) * amplitude;
            amplitude *= 0.5f;
            scale *= 2.0f;
        }

        return value;
    }
}