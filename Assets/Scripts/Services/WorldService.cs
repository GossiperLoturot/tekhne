using System;
using System.Threading;

public class WorldService : IDisposable
{
    private static readonly TileService _tile;
    private static readonly EntityService _entity;
    private static readonly GenerationService _generation;
    private static readonly UpdateService _update;
    private static readonly Mutex _mutex;

    public static WorldService current { get; private set; }

    public readonly TileService tile;
    public readonly EntityService entity;
    public readonly GenerationService generation;
    public readonly UpdateService update;

    static WorldService()
    {
        _tile = new TileService();
        _entity = new EntityService();
        _generation = new GenerationService();
        _update = new UpdateService();
        _mutex = new Mutex();
    }

    public WorldService()
    {
        _mutex.WaitOne();
        tile = _tile;
        entity = _entity;
        generation = _generation;
        update = _update;
        current = this;
    }

    public void Dispose()
    {
        current = null;
        _mutex.ReleaseMutex();
    }
}