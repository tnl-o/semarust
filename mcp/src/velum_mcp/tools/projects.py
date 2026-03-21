"""MCP tools — Projects CRUD."""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def list_projects() -> list[dict[str, Any]]:
    """List all Velum projects the current token has access to."""
    client = get_client()
    return await client.get("/projects") or []


async def get_project(project_id: int) -> dict[str, Any]:
    """Get full details of a specific project by ID."""
    client = get_client()
    return await client.get(f"/projects/{project_id}")


async def create_project(
    name: str,
    alert: bool = False,
    alert_chat: str | None = None,
    max_parallel_tasks: int = 0,
) -> dict[str, Any]:
    """Create a new Velum project.

    Args:
        name: Project display name.
        alert: Enable task failure alerts.
        alert_chat: Chat/Telegram ID for alerts.
        max_parallel_tasks: Max tasks running simultaneously (0 = unlimited).
    """
    client = get_client()
    body: dict[str, Any] = {
        "name": name,
        "alert": alert,
        "max_parallel_tasks": max_parallel_tasks,
    }
    if alert_chat:
        body["alert_chat"] = alert_chat
    return await client.post("/projects", body)


async def update_project(
    project_id: int,
    name: str | None = None,
    alert: bool | None = None,
    alert_chat: str | None = None,
    max_parallel_tasks: int | None = None,
) -> dict[str, Any]:
    """Update an existing project's settings."""
    client = get_client()
    current = await client.get(f"/projects/{project_id}")
    body = {
        "name": name if name is not None else current["name"],
        "alert": alert if alert is not None else current.get("alert", False),
        "alert_chat": alert_chat if alert_chat is not None else current.get("alert_chat", ""),
        "max_parallel_tasks": max_parallel_tasks
        if max_parallel_tasks is not None
        else current.get("max_parallel_tasks", 0),
    }
    return await client.put(f"/projects/{project_id}", body)


async def delete_project(project_id: int) -> str:
    """Delete a project and all its resources. This action is irreversible."""
    client = get_client()
    await client.delete(f"/projects/{project_id}")
    return f"Project {project_id} deleted successfully."
