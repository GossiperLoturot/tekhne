public interface IItemStorage
{
    public bool CheckInsert(IItem item);

    public void Insert(IItem item);
}

public class ItemStorage : IItemStorage
{
    private readonly IItem[] items;

    public ItemStorage(int slotCount)
    {
        items = new IItem[slotCount];
    }

    public bool CheckInsert(IItem item)
    {
        for (var i = 0; i < items.Length; i++)
        {
            if (items[i] == null)
            {
                return true;
            }
        }

        return false;
    }

    public void Insert(IItem item)
    {
        for (var i = 0; i < items.Length; i++)
        {
            if (items[i] == null)
            {
                items[i] = item;
                return;
            }
        }
    }
}
