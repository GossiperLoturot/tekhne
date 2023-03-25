using NUnit.Framework;
using UnityEngine;

public class TileTests
{
    [Test]
    public void TileEqual()
    {
        var left = new Tile(Vector3Int.zero, "TEST_RESOURCE");
        var right = new Tile(Vector3Int.zero, "TEST_RESOURCE");

        Assert.AreEqual(left, right);
    }

    [Test]
    public void TileNotEqual_A()
    {
        var left = new Tile(Vector3Int.zero, "TEST_RESOURCE");
        var right = new Tile(Vector3Int.one, "TEST_RESOURCE");

        Assert.AreNotEqual(left, right);
    }

    [Test]
    public void TileNotEqual_B()
    {
        var left = new Tile(Vector3Int.zero, "TEST_RESOURCE");
        var right = new Tile(Vector3Int.zero, "TEST_RESOURCE_OTHER");

        Assert.AreNotEqual(left, right);
    }
}