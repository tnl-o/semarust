"""MCP tools — Audit log and activity events (unique to Velum MCP)."""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def get_audit_log(
    project_id: int | None = None,
    limit: int = 50,
) -> list[dict[str, Any]]:
    """Retrieve the audit log showing who did what and when.

    Each entry contains: id, user, action, object_type, object_id, created.

    Args:
        project_id: Filter to a specific project (None = global admin log).
        limit: Maximum number of entries to return.
    """
    client = get_client()
    if project_id is not None:
        entries = await client.get(f"/project/{project_id}/audit-log") or []
    else:
        entries = await client.get("/audit-log") or []
    return entries[:limit]


async def get_project_events(project_id: int, limit: int = 30) -> list[dict[str, Any]]:
    """Get recent events for a project (task runs, config changes, etc.).

    Args:
        project_id: Project to query.
        limit: Maximum number of events.
    """
    client = get_client()
    events = await client.get(f"/project/{project_id}/events") or []
    return events[:limit]


async def get_system_info() -> dict[str, Any]:
    """Get Velum system information: version, uptime, database type, runner count."""
    client = get_client()
    return await client.get("/info")
