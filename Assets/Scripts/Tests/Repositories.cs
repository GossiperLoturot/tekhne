using Models;
using NUnit.Framework;

namespace Repositories.Tests
{
    public class TileRepositoryTests
    {
        [Test]
        public void TestSetTile()
        {
            var repo = new TileRepository();
            repo.SetTile(new Tile(0, 0, 0, TileKind.SurfaceGrass));
        }

        [Test]
        public void TestGetTile()
        {
            var repo = new TileRepository();
            var tile = repo.GetTile(0, 0, 0);
        }
    }
}
