using NUnit.Framework;
using UnityEngine;

public class TileServiceTests
{
    [Test]
    public void AddTile()
    {
        var service = new TileService();
        service.AddTile(new Tile(Vector3Int.zero, "TEST_RESOURCE"));
        var tile = service.GetTile(Vector3Int.zero);

        Assert.AreEqual(tile, new Tile(Vector3Int.zero, "TEST_RESOURCE"));
    }

    [Test]
    public void UpdateTile()
    {
        var service = new TileService();
        service.AddTile(new Tile(Vector3Int.zero, "TEST_RESOURCE"));
        service.UpdateTile(new Tile(Vector3Int.zero, "TEST_RESOURCE_OTHER"));
        var tile = service.GetTile(Vector3Int.zero);

        Assert.AreEqual(tile, new Tile(Vector3Int.zero, "TEST_RESOURCE_OTHER"));
    }

    [Test]
    public void RemoveTile()
    {
        var service = new TileService();
        service.AddTile(new Tile(Vector3Int.zero, "TEST_RESOURCE"));
        service.RemoveTile(Vector3Int.zero);
        var contains = service.ContainsTile(Vector3Int.zero);

        Assert.AreEqual(contains, false);
    }
}
