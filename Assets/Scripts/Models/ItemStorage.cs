public interface IItemStorage
{
    public bool TryInsert(IItem item);
    public bool TryExtract(IItem item);
}

public class ItemStorage : IItemStorage
{
    private readonly IItem[] items;

    public ItemStorage(int slotCount)
    {
        items = new IItem[slotCount];
    }

    public bool TryInsert(IItem item)
    {
        for (var i = 0; i < items.Length; i++)
        {
            if (items[i] == null)
            {
                items[i] = item;
                return true;
            }
        }
        return false;
    }

    public bool TryExtract(IItem item)
    {
        for (var i = 0; i < items.Length; i++)
        {
            if (items[i] == item)
            {
                items[i] = null;
                return true;
            }
        }
        return false;
    }
}
