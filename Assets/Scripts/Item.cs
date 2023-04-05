using System;

public interface IItem : IEquatable<IItem>
{
    public string resourceName { get; }
}

public class Item : IItem
{
    public string resourceName { get; private set; }

    public Item(string resourceName)
    {
        this.resourceName = resourceName;
    }

    public bool Equals(IItem other)
    {
        return resourceName == other.resourceName;
    }
}
