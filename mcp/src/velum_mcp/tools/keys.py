"""MCP tools — Access Keys (SSH, token, password) CRUD."""

from __future__ import annotations

from typing import Any, Literal

from velum_mcp.client import get_client

KeyType = Literal["ssh", "token", "login_password", "none"]


async def list_access_keys(project_id: int) -> list[dict[str, Any]]:
    """List all access keys (credentials) in a project.

    Note: Secret values are masked — use get_access_key for metadata only.
    """
    client = get_client()
    return await client.get(f"/projects/{project_id}/keys") or []


async def get_access_key(project_id: int, key_id: int) -> dict[str, Any]:
    """Get metadata of a specific access key. Secret values are always masked."""
    client = get_client()
    return await client.get(f"/projects/{project_id}/keys/{key_id}")


async def create_access_key(
    project_id: int,
    name: str,
    key_type: KeyType = "none",
    private_key: str | None = None,
    passphrase: str | None = None,
    username: str | None = None,
    password: str | None = None,
    token: str | None = None,
) -> dict[str, Any]:
    """Create a new access key / credential.

    Args:
        project_id: Target project.
        name: Display name for this key.
        key_type: 'ssh' for SSH private keys, 'token' for API tokens,
                  'login_password' for username+password, 'none' for no auth.
        private_key: PEM-encoded SSH private key (for type='ssh').
        passphrase: SSH key passphrase (for type='ssh', optional).
        username: Username (for type='login_password').
        password: Password (for type='login_password').
        token: API/access token string (for type='token').
    """
    client = get_client()
    body: dict[str, Any] = {
        "project_id": project_id,
        "name": name,
        "type": key_type,
        "override_secret": True,
    }

    if key_type == "ssh":
        body["ssh"] = {
            "private_key": private_key or "",
            "passphrase": passphrase or "",
        }
    elif key_type == "login_password":
        body["login_password"] = {
            "login": username or "",
            "password": password or "",
        }
    elif key_type == "token":
        body["string"] = {"secret": token or ""}

    return await client.post(f"/projects/{project_id}/keys", body)


async def delete_access_key(project_id: int, key_id: int) -> str:
    """Delete an access key from the project."""
    client = get_client()
    await client.delete(f"/projects/{project_id}/keys/{key_id}")
    return f"Access key {key_id} deleted from project {project_id}."
