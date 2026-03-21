"""MCP tools — Cron Schedules management (unique to Velum MCP)."""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def list_schedules(project_id: int) -> list[dict[str, Any]]:
    """List all cron schedules for a project.

    Returns schedule objects with fields: id, name, cron, active, template_id.
    """
    client = get_client()
    return await client.get(f"/projects/{project_id}/schedules") or []


async def get_schedule(project_id: int, schedule_id: int) -> dict[str, Any]:
    """Get details of a specific schedule."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/schedules/{schedule_id}")


async def create_schedule(
    project_id: int,
    template_id: int,
    cron_expression: str,
    name: str = "",
    active: bool = True,
) -> dict[str, Any]:
    """Create a new cron schedule for a template.

    Args:
        project_id: Target project.
        template_id: Template to run on schedule.
        cron_expression: Standard cron expression, e.g. '0 3 * * *' (daily at 3 AM).
        name: Human-readable name for this schedule.
        active: Whether the schedule is enabled immediately.

    Examples:
        '0 * * * *'     — every hour
        '0 2 * * *'     — daily at 2 AM
        '0 0 * * MON'   — weekly on Monday midnight
        '*/15 * * * *'  — every 15 minutes
    """
    client = get_client()
    body: dict[str, Any] = {
        "project_id": project_id,
        "template_id": template_id,
        "cron": cron_expression,
        "name": name or f"Schedule for template {template_id}",
        "active": active,
    }
    return await client.post(f"/projects/{project_id}/schedules", body)


async def toggle_schedule(project_id: int, schedule_id: int, active: bool) -> str:
    """Enable or disable a schedule.

    Args:
        project_id: Project ID.
        schedule_id: Schedule to toggle.
        active: True to enable, False to disable.
    """
    client = get_client()
    await client.put(
        f"/project/{project_id}/schedules/{schedule_id}/active",
        {"active": active},
    )
    state = "enabled" if active else "disabled"
    return f"Schedule {schedule_id} {state}."


async def delete_schedule(project_id: int, schedule_id: int) -> str:
    """Delete a cron schedule."""
    client = get_client()
    await client.delete(f"/projects/{project_id}/schedules/{schedule_id}")
    return f"Schedule {schedule_id} deleted from project {project_id}."


async def validate_cron(cron_expression: str) -> dict[str, Any]:
    """Validate a cron expression and get its human-readable description.

    Args:
        cron_expression: Cron string to validate, e.g. '0 3 * * MON-FRI'.
    """
    client = get_client()
    return await client.post("/projects/1/schedules/validate", {"cron": cron_expression})
