"""AI-powered task failure analysis.

Uses the MCP host's LLM context — when called from Claude, Claude itself
analyses the task output and provides a diagnosis. No external API key needed.
"""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def analyze_task_failure(
    project_id: int,
    task_id: int,
    last_n_lines: int = 100,
) -> dict[str, Any]:
    """Analyse a failed task's output and return a structured diagnosis.

    This tool retrieves the task output and structures it so the AI assistant
    (Claude) can provide a diagnosis within its context window — no external
    API key needed.

    Args:
        project_id: Project containing the task.
        task_id: The failed task to analyse.
        last_n_lines: How many lines of output to include (default 100).

    Returns:
        A dict with: task_metadata, output_tail, analysis_prompt.
        The calling AI should use analysis_prompt to generate the diagnosis.
    """
    client = get_client()

    # Fetch task metadata
    task = await client.get(f"/projects/{project_id}/tasks/{task_id}")
    status = task.get("status", "unknown")

    if status not in ("error", "stopped"):
        return {
            "error": f"Task {task_id} has status '{status}' — analysis is for failed tasks only.",
            "task": task,
        }

    # Fetch output
    try:
        raw = await client.get(f"/projects/{project_id}/tasks/{task_id}/raw_output")
        if isinstance(raw, str):
            lines = raw.splitlines()
        elif isinstance(raw, list):
            lines = [str(r) for r in raw]
        else:
            lines = []
    except Exception:  # noqa: BLE001
        lines = []

    output_tail = "\n".join(lines[-last_n_lines:]) if lines else "(no output captured)"

    return {
        "task_id": task_id,
        "project_id": project_id,
        "template_id": task.get("template_id"),
        "status": status,
        "started": task.get("start"),
        "finished": task.get("end"),
        "build_task": task.get("build_task"),
        "output_lines_total": len(lines),
        "output_tail": output_tail,
        "analysis_prompt": (
            f"The following Velum task (ID {task_id}) failed with status '{status}'.\n"
            f"Template ID: {task.get('template_id')}\n"
            f"Started: {task.get('start')}, Finished: {task.get('end')}\n\n"
            f"Last {min(last_n_lines, len(lines))} lines of output:\n"
            f"```\n{output_tail}\n```\n\n"
            "Please:\n"
            "1. Identify the root cause of the failure\n"
            "2. List the most likely reasons (ranked by probability)\n"
            "3. Provide specific remediation steps\n"
            "4. Suggest any preventive measures for the future"
        ),
    }


async def bulk_analyze_failures(
    project_id: int,
    limit: int = 5,
) -> list[dict[str, Any]]:
    """Fetch and structure the N most recent failed tasks for bulk analysis.

    Args:
        project_id: Project to scan.
        limit: Number of recent failures to include (default 5).

    Returns:
        List of task summaries ready for AI analysis.
    """
    client = get_client()
    tasks: list[dict[str, Any]] = await client.get(f"/projects/{project_id}/tasks") or []
    failed = [t for t in tasks if t.get("status") == "error"]
    # Most recent first
    failed = sorted(failed, key=lambda t: t.get("id", 0), reverse=True)[:limit]

    results = []
    for task in failed:
        try:
            summary = await analyze_task_failure(project_id, task["id"], last_n_lines=50)
            results.append(summary)
        except Exception as e:  # noqa: BLE001
            results.append({"task_id": task["id"], "error": str(e)})

    return results
