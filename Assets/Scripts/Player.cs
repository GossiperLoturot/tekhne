using System;
using UnityEngine;
using UnityEngine.InputSystem;

public class Player : MonoBehaviour
{
    public Camera camera;
    public float moveSpeed;

    private Vector2 moveValue;
    private Vector2 pointerValue;

    private void Update()
    {
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

            var rawPos = camera.ScreenToWorldPoint(pointerValue);
            var pos = Vector3Int.RoundToInt(new Vector3(rawPos.x, rawPos.y));

            if (WorldService.tile.ContainsTile(pos))
            {
                WorldService.tile.RemoveTile(pos);
            }
        }
    }
}
