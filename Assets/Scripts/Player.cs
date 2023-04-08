using System;
using UnityEngine;
using UnityEngine.InputSystem;

public class Player : MonoBehaviour
{
    private const float PICK_DISTANCE = 3.0f;

    public Camera camera;
    public float moveSpeed;

    private Vector2 moveValue;
    private Vector2 pointerValue;

    private void Update()
    {
        var center = Vector3Int.FloorToInt(transform.position);
        var extent = Vector3Int.one * Mathf.CeilToInt(PICK_DISTANCE);

        var bounds = new BoundsInt();
        bounds.SetMinMax(center - extent, center + extent);

        var entities = WorldService.entity.GetEntitiesFromBounds(bounds);
        foreach (var entity in entities)
        {
            if (entity is IPickable pickableEntity)
            {
                var sqrDist = Vector3.SqrMagnitude(entity.pos - transform.position);
                if (sqrDist <= PICK_DISTANCE * PICK_DISTANCE)
                {
                    pickableEntity.Pick();
                }
            }
        }

        transform.position += new Vector3(moveValue.x, moveValue.y) * moveSpeed * Time.deltaTime;
    }

    public void MoveCallback(InputAction.CallbackContext cx)
    {
        moveValue = cx.ReadValue<Vector2>();
    }

    public void PointerCallback(InputAction.CallbackContext cx)
    {
        pointerValue = cx.ReadValue<Vector2>();
    }

    public void PrimaryCallback(InputAction.CallbackContext cx)
    {
        if (cx.performed)
        {
            if (camera == null)
            {
                throw new Exception("camera is null. please set camera in inspector");
            }

            var pos = camera.ScreenToWorldPoint(pointerValue);
            var hit = Physics2D.OverlapPoint(pos);

            if (hit != null)
            {
                var customProperty = hit.GetComponentInParent<ICustomProperty>();

                switch (customProperty)
                {
                    case CustomPropertyTile prop:
                        WorldService.mutex.WaitOne();
                        var tile = WorldService.tile.GetTile(prop.value);
                        if (tile is IHarvestable tileHarvestable)
                        {
                            tileHarvestable.OnHarvest();
                        }
                        WorldService.mutex.ReleaseMutex();
                        break;

                    case CustomPropertyEntity prop:
                        WorldService.mutex.WaitOne();
                        var entity = WorldService.entity.GetEntity(prop.value);
                        if (entity is IHarvestable entityHarvestable)
                        {
                            entityHarvestable.OnHarvest();
                        }
                        WorldService.mutex.ReleaseMutex();
                        break;
                }
            }
        }
    }
}
