using System.Collections.Generic;
using UnityEngine;

public class EntityService
{
    private readonly Dictionary<string, IEntity> entities;

    private BoundsInt? updateBounds;
    private Queue<IEntityCommand> updateCommands;

    public EntityService()
    {
        entities = new();
        updateCommands = new();
    }

    public void AddEntity(IEntity entity)
    {
        entities.Add(entity.id, entity);

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.Contains(entity.pos))
            {
                updateCommands.Enqueue(new AddEntityCommand(entity));
            }
        }
    }

    public void UpdateEntity(IEntity entity)
    {
        var prev = entities[entity.id];

        entities[entity.id] = entity;

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.Contains(prev.pos))
            {
                updateCommands.Enqueue(new RemoveEntityCommand(prev.id));
            }

            if (updateBounds.Contains(entity.pos))
            {
                updateCommands.Enqueue(new AddEntityCommand(entity));
            }
        }
    }

    public void RemoveEntity(string id)
    {
        var entity = entities[id];

        entities.Remove(entity.id);

        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds.Contains(entity.pos))
            {
                updateCommands.Enqueue(new AddEntityCommand(entity));
            }
        }
    }

    public IEntity GetEntity(string id)
    {
        return entities[id];
    }

    public void SetUpdateBounds(BoundsInt bounds)
    {
        if (this.updateBounds is BoundsInt updateBounds)
        {
            if (updateBounds != bounds)
            {
                foreach (var entity in entities.Values)
                {
                    if (updateBounds.Contains(entity.pos) && !bounds.Contains(entity.pos))
                    {
                        updateCommands.Enqueue(new RemoveEntityCommand(entity.id));
                    }

                    if (!updateBounds.Contains(entity.pos) && bounds.Contains(entity.pos))
                    {
                        updateCommands.Enqueue(new AddEntityCommand(entity));
                    }
                }

                this.updateBounds = bounds;
            }
        }
        else
        {
            foreach (var entity in entities.Values)
            {
                if (bounds.Contains(entity.pos))
                {
                    updateCommands.Enqueue(new AddEntityCommand(entity));
                }
            }

            this.updateBounds = bounds;
        }
    }

    public Queue<IEntityCommand> GetUpdateCommands()
    {
        var cmds = updateCommands;
        updateCommands = new();
        return cmds;
    }

    public interface IEntityCommand { }

    public class AddEntityCommand : IEntityCommand
    {
        public IEntity entity { get; private set; }

        public AddEntityCommand(IEntity entity)
        {
            this.entity = entity;
        }
    }

    public class RemoveEntityCommand : IEntityCommand
    {
        public string id { get; private set; }

        public RemoveEntityCommand(string id)
        {
            this.id = id;
        }
    }
}