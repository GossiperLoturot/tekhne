using NUnit.Framework;
using UnityEngine;

public class BoundsIntExtTests
{
    [Test]
    public void InclusiveContains_InBounds()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3Int(1, 2, 3);
        Assert.AreEqual(bounds.InclusiveContains(point), true);
    }

    [Test]
    public void InclusiveContains_OnBorderA()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3Int(0, 0, 0);
        Assert.AreEqual(bounds.InclusiveContains(point), true);
    }

    [Test]
    public void InclusiveContains_OnBorderB()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3Int(8, 8, 8);
        Assert.AreEqual(bounds.InclusiveContains(point), true);
    }

    [Test]
    public void InclusiveContains_OutOfBounds()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3Int(9, 10, 11);
        Assert.AreEqual(bounds.InclusiveContains(point), false);
    }

    [Test]
    public void Contains_InBounds()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3(1.0f, 2.0f, 3.0f);
        Assert.AreEqual(bounds.Contains(point), true);
    }

    [Test]
    public void Contains_OnBorderA()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3(0.0f, 0.0f, 0.0f);
        Assert.AreEqual(bounds.Contains(point), true);
    }

    [Test]
    public void Contains_OnBorderB()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3(8.0f, 8.0f, 8.0f);
        Assert.AreEqual(bounds.Contains(point), true);
    }

    [Test]
    public void Contains_OutOfBounds()
    {
        var bounds = new BoundsInt(new Vector3Int(0, 0, 0), new Vector3Int(8, 8, 8));
        var point = new Vector3(9.0f, 10.0f, 11.0f);
        Assert.AreEqual(bounds.Contains(point), false);
    }
}
