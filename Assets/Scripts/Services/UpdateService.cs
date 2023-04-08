using System;
using System.Collections.Generic;

public class UpdateService
{
    private readonly Dictionary<string, IUpdatable> updatables;

    public UpdateService()
    {
        updatables = new();
    }

    public void AddUpdatable(IUpdatable updatable)
    {
        if (updatables.ContainsKey(updatable.id))
        {
            throw new Exception("updatable is already existed");
        }

        updatables.Add(updatable.id, updatable);
    }

    public void RemoveUpdatable(string id)
    {
        if (!updatables.ContainsKey(id))
        {
            throw new Exception("updatable is not found");
        }

        updatables.Remove(id);
    }

    public void DispatchUpdateEvent()
    {
        var updatables = new List<IUpdatable>(this.updatables.Values);

        foreach (var updatable in updatables)
        {
            updatable.OnUpdate();
        }
    }

    public interface IUpdatable
    {
        public string id { get; }

        public void OnUpdate();
    }
}