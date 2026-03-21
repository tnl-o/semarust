"""Velum API HTTP client — typed wrapper around the REST API."""

from __future__ import annotations

import os
from typing import Any

import httpx


class VelumClient:
    """Async HTTP client for the Velum REST API.

    Configuration via environment variables:
        VELUM_URL           — base URL, e.g. http://localhost:8088  (required)
        VELUM_API_TOKEN     — API token from User Settings           (required)
        VELUM_TIMEOUT       — request timeout in seconds             (default 30)
    """

    def __init__(self) -> None:
        base_url = os.environ.get("VELUM_URL", "").rstrip("/")
        token = os.environ.get("VELUM_API_TOKEN", "")

        if not base_url:
            raise ValueError("VELUM_URL environment variable is required")
        if not token:
            raise ValueError("VELUM_API_TOKEN environment variable is required")

        timeout = float(os.environ.get("VELUM_TIMEOUT", "30"))

        self._client = httpx.AsyncClient(
            base_url=f"{base_url}/api",
            headers={
                "Authorization": f"Bearer {token}",
                "Content-Type": "application/json",
                "Accept": "application/json",
            },
            timeout=timeout,
        )

    async def get(self, path: str, **params: Any) -> Any:
        r = await self._client.get(path, params={k: v for k, v in params.items() if v is not None})
        r.raise_for_status()
        return r.json() if r.content else None

    async def post(self, path: str, body: dict[str, Any] | None = None) -> Any:
        r = await self._client.post(path, json=body or {})
        r.raise_for_status()
        return r.json() if r.content else None

    async def put(self, path: str, body: dict[str, Any] | None = None) -> Any:
        r = await self._client.put(path, json=body or {})
        r.raise_for_status()
        return r.json() if r.content else None

    async def delete(self, path: str) -> None:
        r = await self._client.delete(path)
        r.raise_for_status()

    async def close(self) -> None:
        await self._client.aclose()


# Module-level singleton — initialised lazily on first tool call
_client: VelumClient | None = None


def get_client() -> VelumClient:
    global _client
    if _client is None:
        _client = VelumClient()
    return _client
