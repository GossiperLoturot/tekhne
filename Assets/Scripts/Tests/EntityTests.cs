using NUnit.Framework;
using UnityEngine;

public class EntityTests
{
    [Test]
    public void EntityEqual()
    {
        var left = new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE");
        var right = new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE");

        Assert.AreEqual(left, right);
    }

    [Test]
    public void EntityNotEqual_A()
    {
        var left = new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE");
        var right = new Entity("TEST_ID_OTHER", Vector3.zero, "TEST_RESOURCE");

        Assert.AreNotEqual(left, right);
    }

    [Test]
    public void EntityNotEqual_B()
    {
        var left = new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE");
        var right = new Entity("TEST_ID", Vector3.one, "TEST_RESOURCE");

        Assert.AreNotEqual(left, right);
    }

    [Test]
    public void EntityNotEqual_C()
    {
        var left = new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE");
        var right = new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE_OTHER");

        Assert.AreNotEqual(left, right);
    }
}
