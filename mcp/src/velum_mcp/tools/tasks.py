"""MCP tools — Task management, execution, output, filtering."""

from __future__ import annotations

from typing import Any, Literal

from velum_mcp.client import get_client


async def list_tasks(
    project_id: int,
    limit: int = 20,
    status: Literal["waiting", "running", "success", "error", "stopped"] | None = None,
) -> list[dict[str, Any]]:
    """List recent tasks in a project, optionally filtered by status.

    Args:
        project_id: Project to query.
        limit: Maximum number of tasks to return (default 20).
        status: Filter by task status (optional).
    """
    client = get_client()
    tasks: list[dict[str, Any]] = await client.get(f"/projects/{project_id}/tasks") or []
    if status:
        tasks = [t for t in tasks if t.get("status") == status]
    return tasks[:limit]


async def get_task(project_id: int, task_id: int) -> dict[str, Any]:
    """Get full details of a specific task including status, timing, and metadata."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/tasks/{task_id}")


async def run_task(
    project_id: int,
    template_id: int,
    message: str = "",
    git_branch: str | None = None,
    extra_vars: str | None = None,
    inventory_id: int | None = None,
) -> dict[str, Any]:
    """Run a task from a template. Returns the new task object with its ID.

    Args:
        project_id: Project ID.
        template_id: Template to run.
        message: Human-readable note for this run (shows in history).
        git_branch: Override branch/tag/commit from template defaults.
        extra_vars: JSON or YAML extra_vars override string.
        inventory_id: Override the template's default inventory.

    Returns:
        Task object — use task_id to poll status or stream logs.
    """
    client = get_client()
    body: dict[str, Any] = {"template_id": template_id}
    if message:
        body["message"] = message
    if git_branch:
        body["git_branch"] = git_branch
    if extra_vars:
        body["arguments"] = extra_vars
    if inventory_id is not None:
        body["inventory_id"] = inventory_id
    return await client.post(f"/project/{project_id}/tasks", body)


async def stop_task(project_id: int, task_id: int) -> str:
    """Send a stop signal to a running task.

    Args:
        project_id: Project ID.
        task_id: Task to stop.
    """
    client = get_client()
    await client.post(f"/projects/{project_id}/tasks/{task_id}/stop")
    return f"Stop signal sent to task {task_id}."


async def get_task_output(
    project_id: int,
    task_id: int,
    raw: bool = False,
    last_n_lines: int | None = None,
) -> str:
    """Retrieve the console output of a completed or running task.

    Args:
        project_id: Project ID.
        task_id: Task to read output from.
        raw: If True, return raw unformatted text; otherwise formatted records.
        last_n_lines: Return only the last N lines of output (optional).
    """
    client = get_client()
    path = f"/projects/{project_id}/tasks/{task_id}/raw_output" if raw else \
           f"/projects/{project_id}/tasks/{task_id}/output"
    result = await client.get(path)

    if isinstance(result, str):
        lines = result.splitlines()
    elif isinstance(result, list):
        lines = [r.get("output", "") for r in result]
    else:
        lines = [str(result)]

    if last_n_lines:
        lines = lines[-last_n_lines:]
    return "\n".join(lines)


async def filter_tasks(
    project_id: int,
    template_id: int | None = None,
    status: Literal["waiting", "running", "success", "error", "stopped"] | None = None,
    limit: int = 50,
) -> list[dict[str, Any]]:
    """Filter project tasks by template and/or status.

    Args:
        project_id: Project to query.
        template_id: Filter to tasks from a specific template (optional).
        status: Filter by execution status (optional).
        limit: Maximum results.
    """
    client = get_client()
    tasks: list[dict[str, Any]] = await client.get(f"/projects/{project_id}/tasks") or []
    if template_id is not None:
        tasks = [t for t in tasks if t.get("template_id") == template_id]
    if status:
        tasks = [t for t in tasks if t.get("status") == status]
    return tasks[:limit]


async def get_latest_failed_task(project_id: int) -> dict[str, Any] | str:
    """Return the most recent failed task in a project, or a message if none found."""
    client = get_client()
    tasks: list[dict[str, Any]] = await client.get(f"/projects/{project_id}/tasks") or []
    failed = [t for t in tasks if t.get("status") == "error"]
    if not failed:
        return "No failed tasks found in this project."
    # Sort by ID descending (higher ID = more recent)
    return max(failed, key=lambda t: t.get("id", 0))


async def get_waiting_tasks(project_id: int) -> list[dict[str, Any]]:
    """Return all tasks currently waiting in the queue for a project."""
    client = get_client()
    tasks: list[dict[str, Any]] = await client.get(f"/projects/{project_id}/tasks") or []
    return [t for t in tasks if t.get("status") == "waiting"]


async def bulk_stop_tasks(project_id: int, task_ids: list[int]) -> str:
    """Stop multiple tasks at once.

    Args:
        project_id: Project containing the tasks.
        task_ids: List of task IDs to stop.
    """
    client = get_client()
    results = []
    for tid in task_ids:
        try:
            await client.post(f"/projects/{project_id}/tasks/{tid}/stop")
            results.append(f"✓ {tid}")
        except Exception as e:  # noqa: BLE001
            results.append(f"✗ {tid}: {e}")
    return "Stop results:\n" + "\n".join(results)


async def confirm_task(project_id: int, task_id: int) -> str:
    """Confirm (approve) a task that is waiting for human approval (gated execution)."""
    client = get_client()
    await client.post(f"/projects/{project_id}/tasks/{task_id}/confirm")
    return f"Task {task_id} confirmed and will proceed."


async def reject_task(project_id: int, task_id: int) -> str:
    """Reject a task waiting for human approval — it will be stopped."""
    client = get_client()
    await client.post(f"/projects/{project_id}/tasks/{task_id}/reject")
    return f"Task {task_id} rejected and stopped."
