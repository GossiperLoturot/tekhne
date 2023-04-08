using UnityEngine;

public class CustomPropertyEntity : MonoBehaviour, ICustomProperty
{
    public string value { get; set; }
}