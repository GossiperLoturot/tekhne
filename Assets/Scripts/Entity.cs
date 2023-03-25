using System;
using UnityEngine;

public interface IEntity : IEquatable<IEntity>
{
    public string id { get; }
    public Vector3 pos { get; }
    public string resourceName { get; }
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
}