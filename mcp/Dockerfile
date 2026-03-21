FROM python:3.12-slim

WORKDIR /app

# Install uv for fast dependency resolution
RUN pip install --no-cache-dir uv

COPY pyproject.toml .
COPY src/ src/

# Install the package and its dependencies
RUN uv pip install --system --no-cache .

EXPOSE 8500

ENV MCP_TRANSPORT=http
ENV MCP_HOST=0.0.0.0
ENV MCP_PORT=8500

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s \
    CMD python -c "import httpx; httpx.get('http://localhost:8500/mcp').raise_for_status()" || exit 1

ENTRYPOINT ["python", "-m", "velum_mcp.server"]
