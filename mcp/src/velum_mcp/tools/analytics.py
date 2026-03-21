"""MCP tools — Analytics and metrics (unique to Velum MCP)."""

from __future__ import annotations

from typing import Any, Literal

from velum_mcp.client import get_client

Period = Literal["today", "week", "month", "year"]


async def get_project_analytics(
    project_id: int,
    period: Period = "week",
) -> dict[str, Any]:
    """Get task execution analytics for a project.

    Returns aggregated stats: total runs, success rate, average duration,
    failure count, and per-template breakdown.

    Args:
        project_id: Project to analyse.
        period: Time window — 'today', 'week', 'month', or 'year'.
    """
    client = get_client()
    return await client.get(f"/project/{project_id}/analytics", period=period)


async def get_task_trends(
    project_id: int,
    period: Period = "week",
) -> list[dict[str, Any]]:
    """Get time-series data of task success/failure counts for charting.

    Useful for understanding deployment frequency and reliability trends.

    Args:
        project_id: Project to analyse.
        period: Time window.
    """
    client = get_client()
    return await client.get(f"/project/{project_id}/analytics/tasks-chart", period=period) or []


async def get_system_analytics() -> dict[str, Any]:
    """Get system-wide analytics: total projects, tasks, users, runners."""
    client = get_client()
    return await client.get("/analytics/system")


async def get_project_health(project_id: int) -> dict[str, Any]:
    """Compute a health summary for a project based on recent task history.

    Returns:
        A dict with: success_rate (%), last_failure, consecutive_failures,
        total_runs_week, average_duration_minutes, health_status.
    """
    client = get_client()
    analytics = await client.get(f"/project/{project_id}/analytics", period="week")

    total = analytics.get("total", 0)
    success = analytics.get("success", 0)
    failure = analytics.get("error", 0)

    success_rate = round((success / total * 100) if total > 0 else 0, 1)

    if success_rate >= 95:
        health = "healthy"
    elif success_rate >= 80:
        health = "degraded"
    else:
        health = "critical"

    return {
        "project_id": project_id,
        "period": "week",
        "total_runs": total,
        "success_count": success,
        "failure_count": failure,
        "success_rate_pct": success_rate,
        "health_status": health,
        "raw": analytics,
    }
