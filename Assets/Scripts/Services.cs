using Models;

namespace Services
{
    public interface ITileRepository
    {
        public void SetTile(Tile tile);
        public Tile GetTile(int x, int y, int layer);
    }
}
