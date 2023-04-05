using NUnit.Framework;

public class ItemTests
{
    [Test]
    public void ItemEqual()
    {
        var left = new Item("TEST_RESOURCE");
        var right = new Item("TEST_RESOURCE");

        Assert.AreEqual(left, right);
    }

    [Test]
    public void ItemNotEqual()
    {
        var left = new Item("TEST_RESOURCE");
        var right = new Item("TEST_RESOURCE_OTHER");

        Assert.AreNotEqual(left, right);
    }
}