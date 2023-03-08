using Models;
using Services;
using System;
using System.Collections.Generic;

namespace Repositories
{
    public class TileRepository : ITileRepository
    {
        private const int GROUP_SIZE = 16;
        private const int POOL_SIZE = 256;

        private readonly Dictionary<(int, int), Dictionary<(int, int), Dictionary<int, Tile>>> pool;
        private readonly Queue<(int, int)> queue;

        public TileRepository()
        {
            pool = new();
            queue = new();
        }

        public void SetTile(Tile tile)
        {
            var gx = (int)MathF.Floor((float)tile.x / (float)GROUP_SIZE);
            var gy = (int)MathF.Floor((float)tile.y / (float)GROUP_SIZE);

            PopGroup(gx, gy);
            var group = pool[(gx, gy)];
            var tiles = group[(tile.x, tile.y)];

            if (tiles.ContainsKey(tile.layer))
            {
                tiles[tile.layer] = tile;
            }
            else
            {
                tiles.Add(tile.layer, tile);
            }
        }

        public Tile GetTile(int x, int y, int layer)
        {
            var gx = (int)MathF.Floor((float)x / (float)GROUP_SIZE);
            var gy = (int)MathF.Floor((float)y / (float)GROUP_SIZE);

            PopGroup(gx, gy);
            var group = pool[(gx, gy)];
            var tiles = group[(x, y)];

            if (tiles.ContainsKey(layer))
            {
                return tiles[layer];
            }
            else
            {
                return null;
            }
        }

        private void PopGroup(int gx, int gy)
        {
            if (!pool.ContainsKey((gx, gy)))
            {
                pool.Add((gx, gy), new());
                LoadGroup(gx, gy);
                queue.Enqueue((gx, gy));

                if (POOL_SIZE < pool.Count)
                {
                    var (_gx, _gy) = queue.Dequeue();
                    SaveGroup(_gx, _gy);
                    pool.Remove((_gx, _gy));
                }
            }
        }

        private void LoadGroup(int gx, int gy)
        {
            // TODO
        }

        private void SaveGroup(int gx, int gy)
        {
            // TODO
        }
    }
}
