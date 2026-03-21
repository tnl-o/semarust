"""MCP tools — Git Repositories CRUD."""

from __future__ import annotations

from typing import Any

from velum_mcp.client import get_client


async def list_repositories(project_id: int) -> list[dict[str, Any]]:
    """List all Git repositories configured in a project."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/repositories") or []


async def get_repository(project_id: int, repository_id: int) -> dict[str, Any]:
    """Get details of a specific repository."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/repositories/{repository_id}")


async def create_repository(
    project_id: int,
    name: str,
    git_url: str,
    git_branch: str = "main",
    ssh_key_id: int | None = None,
) -> dict[str, Any]:
    """Add a Git repository to the project.

    Args:
        project_id: Target project.
        name: Display name for the repository.
        git_url: Git clone URL (SSH or HTTPS).
        git_branch: Default branch/tag/commit to checkout.
        ssh_key_id: SSH or access-token key for authentication.
    """
    client = get_client()
    body: dict[str, Any] = {
        "project_id": project_id,
        "name": name,
        "git_url": git_url,
        "git_branch": git_branch,
    }
    if ssh_key_id is not None:
        body["ssh_key_id"] = ssh_key_id
    return await client.post(f"/projects/{project_id}/repositories", body)


async def update_repository(
    project_id: int,
    repository_id: int,
    name: str | None = None,
    git_url: str | None = None,
    git_branch: str | None = None,
) -> dict[str, Any]:
    """Update repository settings."""
    client = get_client()
    current = await client.get(f"/projects/{project_id}/repositories/{repository_id}")
    body = dict(current)
    if name is not None:
        body["name"] = name
    if git_url is not None:
        body["git_url"] = git_url
    if git_branch is not None:
        body["git_branch"] = git_branch
    return await client.put(f"/projects/{project_id}/repositories/{repository_id}", body)


async def delete_repository(project_id: int, repository_id: int) -> str:
    """Remove a repository from the project."""
    client = get_client()
    await client.delete(f"/projects/{project_id}/repositories/{repository_id}")
    return f"Repository {repository_id} deleted from project {project_id}."


async def list_repository_branches(project_id: int, repository_id: int) -> list[str]:
    """List available branches and tags in a repository."""
    client = get_client()
    result = await client.get(f"/project/{project_id}/repositories/{repository_id}/branches")
    if isinstance(result, list):
        return result
    return []
