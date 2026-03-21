"""MCP tools — Environments (variable sets) CRUD."""

from __future__ import annotations

import json
from typing import Any

from velum_mcp.client import get_client


async def list_environments(project_id: int) -> list[dict[str, Any]]:
    """List all environments (variable sets) in a project."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/environments") or []


async def get_environment(project_id: int, environment_id: int) -> dict[str, Any]:
    """Get details of a specific environment including its variables."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/environments/{environment_id}")


async def create_environment(
    project_id: int,
    name: str,
    env_vars: dict[str, str] | None = None,
    extra_vars: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Create a new environment with optional variables.

    Args:
        project_id: Target project.
        name: Environment display name.
        env_vars: Shell environment variables (injected as process env).
        extra_vars: Ansible extra_vars (passed as --extra-vars JSON).
    """
    client = get_client()
    body: dict[str, Any] = {
        "project_id": project_id,
        "name": name,
        "env": json.dumps(env_vars or {}),
        "json": json.dumps(extra_vars or {}),
        "vars": "{}",
    }
    return await client.post(f"/projects/{project_id}/environments", body)


async def update_environment(
    project_id: int,
    environment_id: int,
    name: str | None = None,
    env_vars: dict[str, str] | None = None,
    extra_vars: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Update an environment's name or variables."""
    client = get_client()
    current = await client.get(f"/projects/{project_id}/environments/{environment_id}")
    body = dict(current)
    if name is not None:
        body["name"] = name
    if env_vars is not None:
        body["env"] = json.dumps(env_vars)
    if extra_vars is not None:
        body["json"] = json.dumps(extra_vars)
    return await client.put(f"/projects/{project_id}/environments/{environment_id}", body)


async def delete_environment(project_id: int, environment_id: int) -> str:
    """Delete an environment."""
    client = get_client()
    await client.delete(f"/projects/{project_id}/environments/{environment_id}")
    return f"Environment {environment_id} deleted from project {project_id}."
