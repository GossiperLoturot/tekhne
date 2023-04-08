using UnityEngine;

public class CustomPropertyTile : MonoBehaviour, ICustomProperty
{
    public Vector3Int value { get; set; }
}