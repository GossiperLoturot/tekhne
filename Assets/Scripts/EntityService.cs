using System;
using System.Collections.Generic;
using UnityEngine;

public class EntityService
{
    private const int GROUP_SIZE = 16;

    private readonly Dictionary<string, IEntity> entities;
    private readonly Dictionary<Vector3Int, HashSet<string>> groupIndex;

    private BoundsInt? updateGroupBounds;
    private Queue<IEntityCommand> updateCommands;

    public EntityService()
    {
        entities = new();
        groupIndex = new();
        updateCommands = new();
    }

    public void AddEntity(IEntity entity)
    {
        if (entities.ContainsKey(entity.id))
        {
            throw new Exception("entity is already existed");
        }

        entities.Add(entity.id, entity);
        entity.OnAfterAdd();

        var group = Vector3Int.FloorToInt(entity.pos / GROUP_SIZE);
        if (!groupIndex.ContainsKey(group))
        {
            groupIndex.Add(group, new());
        }
        groupIndex[group].Add(entity.id);

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds.InclusiveContains(group))
            {
                updateCommands.Enqueue(new AddEntityCommand(entity));
            }
        }
    }

    public void UpdateEntity(IEntity entity)
    {
        if (!entities.ContainsKey(entity.id))
        {
            throw new Exception("entity is not founded");
        }

        var prev = entities[entity.id];
        prev.OnBeforeRemove();
        entities[entity.id] = entity;
        entity.OnAfterAdd();

        var prevGroup = Vector3Int.FloorToInt(prev.pos / GROUP_SIZE);
        groupIndex[prevGroup].Remove(prev.id);
        if (groupIndex[prevGroup].Count == 0)
        {
            groupIndex.Remove(prevGroup);
        }

        var group = Vector3Int.FloorToInt(entity.pos / GROUP_SIZE);
        if (!groupIndex.ContainsKey(group))
        {
            groupIndex.Add(group, new());
        }
        groupIndex[group].Add(entity.id);

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds.InclusiveContains(prevGroup))
            {
                updateCommands.Enqueue(new RemoveEntityCommand(prev.id));
            }

            if (updateGroupBounds.InclusiveContains(group))
            {
                updateCommands.Enqueue(new AddEntityCommand(entity));
            }
        }
    }

    public void RemoveEntity(string id)
    {
        if (!entities.ContainsKey(id))
        {
            throw new Exception("entity is not founded");
        }

        var entity = entities[id];
        entity.OnBeforeRemove();
        entities.Remove(entity.id);

        var group = Vector3Int.FloorToInt(entity.pos / GROUP_SIZE);
        groupIndex[group].Remove(entity.id);
        if (groupIndex[group].Count == 0)
        {
            groupIndex.Remove(group);
        }

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds.InclusiveContains(group))
            {
                updateCommands.Enqueue(new RemoveEntityCommand(entity.id));
            }
        }
    }

    public bool ContainsEntity(string id)
    {
        return entities.ContainsKey(id);
    }

    public IEntity GetEntity(string id)
    {
        if (!entities.ContainsKey(id))
        {
            throw new Exception("entity is not founded");
        }

        return entities[id];
    }

    public void SetUpdateBounds(BoundsInt bounds)
    {
        var groupBounds = new BoundsInt();
        var min = Vector3Int.FloorToInt((Vector3)bounds.min / GROUP_SIZE);
        var max = Vector3Int.FloorToInt((Vector3)bounds.max / GROUP_SIZE);
        groupBounds.SetMinMax(min, max);

        if (this.updateGroupBounds is BoundsInt updateGroupBounds)
        {
            if (updateGroupBounds != groupBounds)
            {
                for (var x = updateGroupBounds.xMin; x <= updateGroupBounds.xMax; x++)
                {
                    for (var y = updateGroupBounds.yMin; y <= updateGroupBounds.yMax; y++)
                    {
                        for (var z = updateGroupBounds.zMin; z <= updateGroupBounds.zMax; z++)
                        {
                            var group = new Vector3Int(x, y, z);
                            if (!groupBounds.InclusiveContains(group))
                            {
                                if (groupIndex.ContainsKey(group))
                                {
                                    foreach (var id in groupIndex[group])
                                    {
                                        updateCommands.Enqueue(new RemoveEntityCommand(id));
                                    }
                                }
                            }
                        }
                    }
                }

                for (var x = groupBounds.xMin; x <= groupBounds.xMax; x++)
                {
                    for (var y = groupBounds.yMin; y <= groupBounds.yMax; y++)
                    {
                        for (var z = groupBounds.zMin; z <= groupBounds.zMax; z++)
                        {
                            var group = new Vector3Int(x, y, z);
                            if (!updateGroupBounds.InclusiveContains(group))
                            {
                                if (groupIndex.ContainsKey(group))
                                {
                                    foreach (var id in groupIndex[group])
                                    {
                                        var entity = entities[id];
                                        updateCommands.Enqueue(new AddEntityCommand(entity));
                                    }
                                }
                            }
                        }
                    }
                }

                this.updateGroupBounds = groupBounds;
            }
        }
        else
        {
            for (var x = groupBounds.xMin; x <= groupBounds.xMax; x++)
            {
                for (var y = groupBounds.yMin; y <= groupBounds.yMax; y++)
                {
                    for (var z = groupBounds.zMin; z <= groupBounds.zMax; z++)
                    {
                        var group = new Vector3Int(x, y, z);
                        if (groupIndex.ContainsKey(group))
                        {
                            foreach (var id in groupIndex[group])
                            {
                                var entity = entities[id];
                                updateCommands.Enqueue(new AddEntityCommand(entity));
                            }
                        }
                    }
                }
            }

            this.updateGroupBounds = groupBounds;
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