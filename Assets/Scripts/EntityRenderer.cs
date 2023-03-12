using System.Collections.Generic;
using UnityEngine;

public class EntityRenderer : MonoBehaviour
{
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

        var entities = Service.entity.GetEntities();

        foreach (var entity in entities)
        {
            var prefab = Resources.Load<GameObject>(entity.resourceName);
            var instance = Instantiate(prefab, new Vector3(entity.x, entity.y, entity.layer), Quaternion.identity);

            instances.Add(instance);
        }
    }
}
