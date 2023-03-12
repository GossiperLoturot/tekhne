using System.Collections.Generic;

public class EntityService
{
    private readonly List<Entity> entities;

    public EntityService()
    {
        entities = new();
    }

    public void AddEntity(Entity entity)
    {
        entities.Add(entity);
    }

    public IEnumerable<Entity> GetEntities()
    {
        return entities;
    }

    public void RemoveEntity(Entity entity)
    {
        entities.Remove(entity);
    }
}
