using UnityEngine;

public static class BoundsIntExt
{
    public static bool InclusiveContains(this BoundsInt bounds, Vector3Int point)
    {
        var isOutOfBounds = point.x < bounds.xMin || bounds.xMax < point.x
            || point.y < bounds.yMin || bounds.yMax < point.y
            || point.z < bounds.zMin || bounds.zMax < point.z;

        return !isOutOfBounds;
    }

    public static bool Contains(this BoundsInt bounds, Vector3 point)
    {
        var isOutOfBounds = point.x < bounds.xMin || bounds.xMax < point.x
            || point.y < bounds.yMin || bounds.yMax < point.y
            || point.z < bounds.zMin || bounds.zMax < point.z;

        return !isOutOfBounds;
    }
}