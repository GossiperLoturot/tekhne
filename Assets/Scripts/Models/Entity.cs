using System;
using UnityEngine;

public interface IEntity : IEquatable<IEntity>
{
    public string id { get; }
    public Vector3 pos { get; }
    public string resourceName { get; }

    public void OnAfterAdd();

    public void OnBeforeRemove();
}

public class Entity : IEntity
{
    public string id { get; private set; }
    public Vector3 pos { get; private set; }
    public string resourceName { get; private set; }

    public Entity(string id, Vector3 pos, string resourceName)
    {
        this.id = id;
        this.pos = pos;
        this.resourceName = resourceName;
    }

    public bool Equals(IEntity other)
    {
        return id == other.id && pos == other.pos && resourceName == other.resourceName;
    }

    public void OnAfterAdd() { }

    public void OnBeforeRemove() { }
}

public class EntityPickable : Entity, IPickable
{
    public IItem item { get; private set; }

    public EntityPickable(string id, Vector3 pos, IItem item) : base(id, pos, item.resourceName)
    {
        this.item = item;
    }

    public void Pick()
    {
        WorldService.entity.RemoveEntity(id);
    }
}

public class EntityCircularMotion : IEntity,  UpdateService.IUpdatable
{
    public string id { get; private set; }
    public Vector3 pos { get; private set; }
    public string resourceName { get; private set; }
    public Vector3 centerPos { get; private set; }

    public EntityCircularMotion(string id, Vector3 pos, string resourceName, Vector3 centerPos)
    {
        this.id = id;
        this.pos = pos;
        this.resourceName = resourceName;
        this.centerPos = centerPos;
    }

    public bool Equals(IEntity other)
    {
        return id == other.id && pos == other.pos && resourceName == other.resourceName;
    }

    public void OnAfterAdd()
    {
        WorldService.update.AddUpdatable(this);
    }

    public void OnBeforeRemove()
    {
        WorldService.update.RemoveUpdatable(id);
    }

    public void OnUpdate()
    {
        var time = Time.time;
        var pos = centerPos + new Vector3(Mathf.Cos(time), Mathf.Sin(time));
        var entity = new EntityCircularMotion(id, pos, resourceName, centerPos);
        WorldService.entity.UpdateEntity(entity);
    }
}