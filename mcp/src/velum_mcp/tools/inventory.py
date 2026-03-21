"""MCP tools — Inventory CRUD."""

from __future__ import annotations

from typing import Any, Literal

from velum_mcp.client import get_client

InventoryType = Literal["static", "file", "dynamic-azure", "dynamic-aws", "dynamic-gcp", "terraform-workspace"]


async def list_inventory(project_id: int) -> list[dict[str, Any]]:
    """List all inventories in a project."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/inventories") or []


async def get_inventory(project_id: int, inventory_id: int) -> dict[str, Any]:
    """Get details of a specific inventory."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/inventories/{inventory_id}")


async def create_inventory(
    project_id: int,
    name: str,
    inventory_type: InventoryType = "static",
    inventory: str = "",
    ssh_key_id: int | None = None,
    become_key_id: int | None = None,
) -> dict[str, Any]:
    """Create a new inventory.

    Args:
        project_id: Target project.
        name: Inventory display name.
        inventory_type: Type — 'static' (YAML/INI text), 'file' (path),
                        'dynamic-aws', 'dynamic-azure', 'dynamic-gcp',
                        or 'terraform-workspace'.
        inventory: Inventory content (for static) or path/config (for others).
        ssh_key_id: SSH key to use for connecting to hosts.
        become_key_id: Become (sudo) key ID (optional).
    """
    client = get_client()
    body: dict[str, Any] = {
        "project_id": project_id,
        "name": name,
        "type": inventory_type,
        "inventory": inventory,
    }
    if ssh_key_id is not None:
        body["ssh_key_id"] = ssh_key_id
    if become_key_id is not None:
        body["become_key_id"] = become_key_id
    return await client.post(f"/projects/{project_id}/inventories", body)


async def update_inventory(
    project_id: int,
    inventory_id: int,
    name: str | None = None,
    inventory: str | None = None,
) -> dict[str, Any]:
    """Update an inventory's name or content."""
    client = get_client()
    current = await client.get(f"/projects/{project_id}/inventories/{inventory_id}")
    body = dict(current)
    if name is not None:
        body["name"] = name
    if inventory is not None:
        body["inventory"] = inventory
    return await client.put(f"/projects/{project_id}/inventories/{inventory_id}", body)


async def delete_inventory(project_id: int, inventory_id: int) -> str:
    """Delete an inventory."""
    client = get_client()
    await client.delete(f"/projects/{project_id}/inventories/{inventory_id}")
    return f"Inventory {inventory_id} deleted from project {project_id}."
