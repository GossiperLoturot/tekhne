using System.Collections.Generic;
using UnityEngine;

public class WorldRenderer : MonoBehaviour
{
    private const int SIZE = 16;
    private const int LAYER_SIZE = 4;

    private Dictionary<Vector3Int, GameObject> tileInstances;
    private Dictionary<string, GameObject> entityInstances;

    private void Start()
    {
        tileInstances = new();
        entityInstances = new();
    }

    private void Update()
    {
        var center = Vector3Int.FloorToInt(transform.position);
        var extent = new Vector3Int(SIZE, SIZE, LAYER_SIZE);
        var bounds = new BoundsInt(center - extent, 2 * extent);
        var boundsf = new Bounds(bounds.center, bounds.size);

        WorldService.generation.SetUpdateBounds(bounds);
        WorldService.tile.SetUpdateBounds(bounds);
        WorldService.entity.SetUpdateBounds(boundsf);

        foreach (var cmd in WorldService.tile.GetUpdateCommands())
        {
            switch (cmd)
            {
                case TileService.AddTileCommand addCmd:
                    var prefab = Resources.Load<GameObject>(addCmd.tile.resourceName);
                    var instance = Instantiate(prefab, addCmd.tile.pos, Quaternion.identity);
                    tileInstances.Add(addCmd.tile.pos, instance);
                    break;

                case TileService.RemoveTileCommand removeCmd:
                    Destroy(tileInstances[removeCmd.pos]);
                    tileInstances.Remove(removeCmd.pos);
                    break;
            }
        }

        foreach (var cmd in WorldService.entity.GetUpdateCommands())
        {
            switch (cmd)
            {
                case EntityService.AddEntityCommand addCmd:
                    var prefab = Resources.Load<GameObject>(addCmd.entity.resourceName);
                    var instance = Instantiate(prefab, addCmd.entity.pos, Quaternion.identity);
                    entityInstances.Add(addCmd.entity.id, instance);
                    break;

                case EntityService.RemoveEntityCommand removeCmd:
                    Destroy(entityInstances[removeCmd.id]);
                    entityInstances.Remove(removeCmd.id);
                    break;
            }
        }
    }
}
