using UnityEngine;

public class PlayerService
{
    private const int MAX_SLOT_COUNT = 10;

    public Vector3 pos { get; private set; }

    public readonly IItemStorage itemStorage;

    public PlayerService()
    {
        itemStorage = new ItemStorage(MAX_SLOT_COUNT);
    }

    public void SetPos(Vector3 pos)
    {
        this.pos = pos;
    }
}