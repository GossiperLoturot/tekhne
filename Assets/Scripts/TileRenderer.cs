using System.Collections.Generic;
using UnityEngine;

public class TileRenderer : MonoBehaviour
{
    private const int SIZE = 16;

    private List<GameObject> instances;

    private void Start()
    {
        instances = new();
    }

    private void Update()
    {
        foreach (var instance in instances)
        {
            Destroy(instance);
        }

        instances.Clear();

        var xCenter = Mathf.FloorToInt(transform.position.x);
        var yCenter = Mathf.FloorToInt(transform.position.y);
        var layerCenter = Mathf.FloorToInt(transform.position.z);

        for (var x = xCenter - SIZE; x <= xCenter + SIZE; x++)
        {
            for (var y = yCenter - SIZE; y <= yCenter + SIZE; y++)
            {
                for (var layer = layerCenter - SIZE; layer < layerCenter + SIZE; layer++)
                {
                    var tile = Context.tile.GetTile(x, y, layer);

                    if (tile != null)
                    {
                        var prefab = Resources.Load<GameObject>(tile.resourceName);
                        var instance = Instantiate(prefab, new Vector3(tile.x, tile.y, tile.layer), Quaternion.identity);

                        instances.Add(instance);
                    }
                }
            }
        }
    }
}
