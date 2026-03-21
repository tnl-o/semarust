"""MCP tools — Runner agents management (unique to Velum MCP)."""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def list_runners() -> list[dict[str, Any]]:
    """List all registered runner agents across all projects.

    Runners are self-registering execution agents that pick up tasks.
    Each runner has: id, name, active, last_active (heartbeat time).
    """
    client = get_client()
    return await client.get("/runners") or []


async def get_runner_status(runner_id: int) -> dict[str, Any]:
    """Get status details of a specific runner, including last heartbeat time."""
    client = get_client()
    runners = await client.get("/runners") or []
    for r in runners:
        if r.get("id") == runner_id:
            return r
    return {"error": f"Runner {runner_id} not found"}


async def toggle_runner(runner_id: int, active: bool) -> str:
    """Enable or disable a runner agent.

    Disabled runners will not pick up new tasks but won't stop running ones.

    Args:
        runner_id: Runner to toggle.
        active: True to enable, False to disable.
    """
    client = get_client()
    runners = await client.get("/runners") or []
    runner = next((r for r in runners if r.get("id") == runner_id), None)
    if not runner:
        return f"Runner {runner_id} not found."
    runner["active"] = active
    await client.put(f"/runners/{runner_id}", runner)
    state = "enabled" if active else "disabled"
    return f"Runner {runner_id} ({runner.get('name', '')}) {state}."


async def clear_runner_cache(runner_id: int) -> str:
    """Clear the Git repository cache on a runner agent.

    Useful when a runner has stale cached content from a repository.
    """
    client = get_client()
    await client.delete(f"/runners/{runner_id}/cache")
    return f"Cache cleared for runner {runner_id}."
