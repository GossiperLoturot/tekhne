using NUnit.Framework;
using UnityEngine;

public class EntityServiceTests
{
    [Test]
    public void AddEntity()
    {
        var service = new EntityService();
        service.AddEntity(new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE"));
        var entity = service.GetEntity("TEST_ID");

        Assert.AreEqual(entity, new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE"));
    }

    [Test]
    public void UpdateEntity_A()
    {
        var service = new EntityService();
        service.AddEntity(new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE"));
        service.UpdateEntity(new Entity("TEST_ID", Vector3.one, "TEST_RESOURCE"));
        var entity = service.GetEntity("TEST_ID");

        Assert.AreEqual(entity, new Entity("TEST_ID", Vector3.one, "TEST_RESOURCE"));
    }

    [Test]
    public void UpdateEntity_B()
    {
        var service = new EntityService();
        service.AddEntity(new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE"));
        service.UpdateEntity(new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE_OTHER"));
        var entity = service.GetEntity("TEST_ID");

        Assert.AreEqual(entity, new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE_OTHER"));
    }

    [Test]
    public void RemoveEntity()
    {
        var service = new EntityService();
        service.AddEntity(new Entity("TEST_ID", Vector3.zero, "TEST_RESOURCE"));
        service.RemoveEntity("TEST_ID");
        var contains = service.ContainsEntity("TEST_ID");

        Assert.AreEqual(contains, false);
    }
}
