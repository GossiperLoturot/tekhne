using System.Collections.Generic;
using UnityEngine;

[RequireComponent(typeof(Camera))]
public class WorldRenderer : MonoBehaviour
{
    private const int LAYER_SIZE = 16;

    private Camera camera;
    private Dictionary<Vector3Int, GameObject> tileInstances;
    private Dictionary<string, GameObject> entityInstances;

    private void Start()
    {
        camera = GetComponent<Camera>();
        tileInstances = new();
        entityInstances = new();
    }

    private void Update()
    {
        var widthExtent = Mathf.CeilToInt(camera.orthographicSize * camera.aspect);
        var heightExtent = Mathf.CeilToInt(camera.orthographicSize);
        var layerExtent = Mathf.CeilToInt(transform.position.z);

        var center = Vector3Int.RoundToInt(new Vector3(transform.position.x, transform.position.y));
        var extent = new Vector3Int(widthExtent, heightExtent, layerExtent);

        var bounds = new BoundsInt();
        bounds.SetMinMax(center - extent, center + extent);

        WorldService.generation.SetUpdateBounds(bounds);
        WorldService.tile.SetUpdateBounds(bounds);
        WorldService.entity.SetUpdateBounds(bounds);

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
