"""MCP tools — Playbook management (unique to Velum MCP)."""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def list_playbooks(project_id: int) -> list[dict[str, Any]]:
    """List all playbooks registered in a project.

    Playbooks are discovered from connected Git repositories.
    """
    client = get_client()
    return await client.get(f"/project/{project_id}/playbooks") or []


async def get_playbook(project_id: int, playbook_id: int) -> dict[str, Any]:
    """Get details of a specific playbook."""
    client = get_client()
    return await client.get(f"/project/{project_id}/playbooks/{playbook_id}")


async def sync_playbook(project_id: int, playbook_id: int) -> str:
    """Synchronize a playbook from its Git repository.

    Pulls the latest version of the playbook file from the configured branch.
    """
    client = get_client()
    await client.post(f"/project/{project_id}/playbooks/{playbook_id}/sync")
    return f"Playbook {playbook_id} synced from Git repository."


async def run_playbook(
    project_id: int,
    playbook_id: int,
    message: str = "",
    git_branch: str | None = None,
) -> dict[str, Any]:
    """Run a playbook directly, creating a task record.

    Args:
        project_id: Project ID.
        playbook_id: Playbook to execute.
        message: Optional human note for this run.
        git_branch: Override branch to checkout before running.

    Returns:
        Playbook run object with run ID for status tracking.
    """
    client = get_client()
    body: dict[str, Any] = {}
    if message:
        body["message"] = message
    if git_branch:
        body["git_branch"] = git_branch
    return await client.post(f"/project/{project_id}/playbooks/{playbook_id}/run", body)


async def get_playbook_history(
    project_id: int,
    playbook_id: int,
    limit: int = 10,
) -> list[dict[str, Any]]:
    """Get the execution history for a specific playbook.

    Args:
        project_id: Project ID.
        playbook_id: Playbook to query.
        limit: Maximum number of runs to return.
    """
    client = get_client()
    runs = await client.get(f"/project/{project_id}/playbook-runs") or []
    filtered = [r for r in runs if r.get("playbook_id") == playbook_id]
    return filtered[:limit]
