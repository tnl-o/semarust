"""MCP tools — Templates (Job Templates) CRUD + run."""

from __future__ import annotations

from typing import Any, Literal

from velum_mcp.client import get_client


async def list_templates(project_id: int) -> list[dict[str, Any]]:
    """List all job templates in a project."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/templates") or []


async def get_template(project_id: int, template_id: int) -> dict[str, Any]:
    """Get full details of a specific template."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/templates/{template_id}")


async def create_template(
    project_id: int,
    name: str,
    playbook: str,
    inventory_id: int,
    repository_id: int,
    environment_id: int,
    ssh_key_id: int | None = None,
    app: Literal["ansible", "terraform", "tofu", "terragrunt", "bash"] = "ansible",
    description: str = "",
    extra_vars: str = "",
    allow_override_args: bool = False,
    allow_override_branch: bool = False,
    suppress_success_alerts: bool = False,
) -> dict[str, Any]:
    """Create a new job template.

    Args:
        project_id: Target project ID.
        name: Template display name.
        playbook: Playbook file path (e.g. 'site.yml') or Terraform directory.
        inventory_id: Inventory to use.
        repository_id: Git repository containing the playbook.
        environment_id: Environment variables to inject.
        ssh_key_id: SSH key for Ansible connection (optional).
        app: Executor type — ansible, terraform, tofu, terragrunt, or bash.
        description: Optional human-readable description.
        extra_vars: JSON or YAML extra variables string.
        allow_override_args: Let users override arguments at run time.
        allow_override_branch: Let users specify a Git branch at run time.
        suppress_success_alerts: Do not send alerts when task succeeds.
    """
    client = get_client()
    body: dict[str, Any] = {
        "project_id": project_id,
        "name": name,
        "playbook": playbook,
        "inventory_id": inventory_id,
        "repository_id": repository_id,
        "environment_id": environment_id,
        "app": app,
        "description": description,
        "arguments": extra_vars,
        "allow_override_args_in_task": allow_override_args,
        "allow_override_branch_in_task": allow_override_branch,
        "suppress_success_alerts": suppress_success_alerts,
    }
    if ssh_key_id is not None:
        body["ssh_key_id"] = ssh_key_id
    return await client.post(f"/projects/{project_id}/templates", body)


async def update_template(
    project_id: int,
    template_id: int,
    name: str | None = None,
    description: str | None = None,
    extra_vars: str | None = None,
    allow_override_args: bool | None = None,
) -> dict[str, Any]:
    """Update selected fields of an existing template."""
    client = get_client()
    current = await client.get(f"/projects/{project_id}/templates/{template_id}")
    body = dict(current)
    if name is not None:
        body["name"] = name
    if description is not None:
        body["description"] = description
    if extra_vars is not None:
        body["arguments"] = extra_vars
    if allow_override_args is not None:
        body["allow_override_args_in_task"] = allow_override_args
    return await client.put(f"/projects/{project_id}/templates/{template_id}", body)


async def delete_template(project_id: int, template_id: int) -> str:
    """Delete a job template."""
    client = get_client()
    await client.delete(f"/projects/{project_id}/templates/{template_id}")
    return f"Template {template_id} deleted from project {project_id}."


async def run_template(
    project_id: int,
    template_id: int,
    message: str = "",
    git_branch: str | None = None,
    extra_vars: str | None = None,
) -> dict[str, Any]:
    """Run a job template immediately — creates and queues a new task.

    Args:
        project_id: Project ID.
        template_id: Template to run.
        message: Optional human-readable note for this run.
        git_branch: Override the default Git branch/tag/commit.
        extra_vars: Override extra variables (JSON/YAML string).

    Returns:
        The created task object with its ID for status tracking.
    """
    client = get_client()
    body: dict[str, Any] = {"template_id": template_id}
    if message:
        body["message"] = message
    if git_branch:
        body["git_branch"] = git_branch
    if extra_vars:
        body["arguments"] = extra_vars
    return await client.post(f"/project/{project_id}/tasks", body)


async def stop_all_template_tasks(project_id: int, template_id: int) -> str:
    """Stop all currently running tasks for a specific template."""
    client = get_client()
    await client.post(f"/projects/{project_id}/templates/{template_id}/stop_all_tasks")
    return f"Stop signal sent to all running tasks of template {template_id}."
