using NUnit.Framework;

public class UpdateServiceTests
{
    public class Updatable : UpdateService.IUpdatable
    {
        public string id { get; private set; }

        public Updatable(string id)
        {
            this.id = id;
        }

        public void OnUpdate() { }
    }

    [Test]
    public void AddUpdatable()
    {
        var service = new UpdateService();
        service.AddUpdatable(new Updatable("TEST_ID"));
    }

    [Test]
    public void RemoveUpdatable()
    {
        var service = new UpdateService();
        service.AddUpdatable(new Updatable("TEST_ID"));
        service.RemoveUpdatable("TEST_ID");
    }
}