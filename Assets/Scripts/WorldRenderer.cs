using System.Collections.Generic;
using UnityEngine;
using UnityEngine.AddressableAssets;

[RequireComponent(typeof(Camera))]
public class WorldRenderer : MonoBehaviour
{
    private const string PRELOAD_LABELS = "preload";

    private Camera camera;
    private Dictionary<string, GameObject> prefabs;
    private Dictionary<Vector3Int, GameObject> tileInstances;
    private Dictionary<string, GameObject> entityInstances;

    private void Start()
    {
        camera = GetComponent<Camera>();
        prefabs = new();
        tileInstances = new();
        entityInstances = new();

        var resourceLocationsHandle = Addressables.LoadResourceLocationsAsync(PRELOAD_LABELS);
        foreach (var resourceLocation in resourceLocationsHandle.WaitForCompletion())
        {
            var prefabHandle = Addressables.LoadAssetAsync<GameObject>(resourceLocation);
            prefabs.Add(resourceLocation.PrimaryKey, prefabHandle.WaitForCompletion());
            Addressables.Release(prefabHandle);
        }
        Addressables.Release(resourceLocationsHandle);
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
        WorldService.update.DispatchUpdateEvent();
        WorldService.tile.SetUpdateBounds(bounds);
        WorldService.entity.SetUpdateBounds(bounds);

        foreach (var cmd in WorldService.tile.GetUpdateCommands())
        {
            switch (cmd)
            {
                case TileService.AddTileCommand addCmd:
                    var prefab = prefabs[addCmd.tile.resourceName];
                    var projPos = Projection(addCmd.tile.pos);
                    var instance = Instantiate(prefab, projPos, Quaternion.identity);
                    var customProperty = instance.AddComponent<CustomPropertyTile>();
                    customProperty.value = addCmd.tile.pos;
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
                    var prefab = prefabs[addCmd.entity.resourceName];
                    var projPos = Projection(addCmd.entity.pos);
                    var instance = Instantiate(prefab, projPos, Quaternion.identity);
                    var customProperty = instance.AddComponent<CustomPropertyEntity>();
                    customProperty.value = addCmd.entity.id;
                    entityInstances.Add(addCmd.entity.id, instance);
                    break;

                case EntityService.RemoveEntityCommand removeCmd:
                    Destroy(entityInstances[removeCmd.id]);
                    entityInstances.Remove(removeCmd.id);
                    break;
            }
        }
    }

    private Vector3 Projection(Vector3 pos)
    {
        return new Vector3(pos.x, pos.y + pos.z, -pos.z);
    }
}
