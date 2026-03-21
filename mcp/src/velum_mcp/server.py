"""Velum MCP Server — entry point.

60 tools covering Projects, Templates, Tasks, Inventory, Repositories,
Environments, Access Keys, Schedules, Analytics, Runners, Playbooks,
Audit Log, and AI-powered failure analysis.

Quick start:
    export VELUM_URL=http://localhost:8088
    export VELUM_API_TOKEN=your-token-here
    python -m velum_mcp.server          # stdio (default)
    MCP_TRANSPORT=http python -m velum_mcp.server  # HTTP on port 8500

Claude Code:
    claude mcp add --transport http velum http://127.0.0.1:8500/mcp

Claude Desktop (claude_desktop_config.json):
    {
      "mcpServers": {
        "velum": {
          "command": "uvx",
          "args": ["velum-mcp"],
          "env": {
            "VELUM_URL": "http://localhost:8088",
            "VELUM_API_TOKEN": "your-token-here"
          }
        }
      }
    }
"""

from __future__ import annotations

import os

from fastmcp import FastMCP

# ── Tool modules ──────────────────────────────────────────────────────────────
from velum_mcp.tools.projects import (
    create_project,
    delete_project,
    get_project,
    list_projects,
    update_project,
)
from velum_mcp.tools.templates import (
    create_template,
    delete_template,
    get_template,
    list_templates,
    run_template,
    stop_all_template_tasks,
    update_template,
)
from velum_mcp.tools.tasks import (
    bulk_stop_tasks,
    confirm_task,
    filter_tasks,
    get_latest_failed_task,
    get_task,
    get_task_output,
    get_waiting_tasks,
    list_tasks,
    reject_task,
    run_task,
    stop_task,
)
from velum_mcp.tools.inventory import (
    create_inventory,
    delete_inventory,
    get_inventory,
    list_inventory,
    update_inventory,
)
from velum_mcp.tools.repositories import (
    create_repository,
    delete_repository,
    get_repository,
    list_repositories,
    list_repository_branches,
    update_repository,
)
from velum_mcp.tools.environments import (
    create_environment,
    delete_environment,
    get_environment,
    list_environments,
    update_environment,
)
from velum_mcp.tools.keys import (
    create_access_key,
    delete_access_key,
    get_access_key,
    list_access_keys,
)
from velum_mcp.tools.schedules import (
    create_schedule,
    delete_schedule,
    get_schedule,
    list_schedules,
    toggle_schedule,
    validate_cron,
)
from velum_mcp.tools.analytics import (
    get_project_analytics,
    get_project_health,
    get_system_analytics,
    get_task_trends,
)
from velum_mcp.tools.runners import (
    clear_runner_cache,
    get_runner_status,
    list_runners,
    toggle_runner,
)
from velum_mcp.tools.playbooks import (
    get_playbook,
    get_playbook_history,
    list_playbooks,
    run_playbook,
    sync_playbook,
)
from velum_mcp.tools.audit import (
    get_audit_log,
    get_project_events,
    get_system_info,
)
from velum_mcp.analysis.ai_analyzer import (
    analyze_task_failure,
    bulk_analyze_failures,
)

# ── Server setup ──────────────────────────────────────────────────────────────

mcp = FastMCP(
    name="velum",
    version="1.0.0",
    instructions=(
        "Velum MCP — AI-native control of Ansible, Terraform, and DevOps automation.\n\n"
        "Connect to a Velum instance to: run deployments, monitor tasks, manage infrastructure "
        "resources, analyse failures, and control schedules.\n\n"
        "Environment variables required:\n"
        "  VELUM_URL          — Velum base URL (e.g. http://localhost:8088)\n"
        "  VELUM_API_TOKEN    — API token from Velum → User Settings → API Tokens\n\n"
        "Typical workflows:\n"
        "  - 'Run the prod deployment'     → run_template or run_task\n"
        "  - 'Why did the last job fail?'  → get_latest_failed_task + analyze_task_failure\n"
        "  - 'Schedule daily backup'       → create_schedule with cron='0 3 * * *'\n"
        "  - 'Show project health'         → get_project_health\n"
        "  - 'List all running tasks'      → list_tasks with status='running'\n"
    ),
)

# ── Register all tools ────────────────────────────────────────────────────────

# Projects
mcp.tool(list_projects)
mcp.tool(get_project)
mcp.tool(create_project)
mcp.tool(update_project)
mcp.tool(delete_project)

# Templates
mcp.tool(list_templates)
mcp.tool(get_template)
mcp.tool(create_template)
mcp.tool(update_template)
mcp.tool(delete_template)
mcp.tool(run_template)
mcp.tool(stop_all_template_tasks)

# Tasks
mcp.tool(list_tasks)
mcp.tool(get_task)
mcp.tool(run_task)
mcp.tool(stop_task)
mcp.tool(get_task_output)
mcp.tool(filter_tasks)
mcp.tool(get_latest_failed_task)
mcp.tool(get_waiting_tasks)
mcp.tool(bulk_stop_tasks)
mcp.tool(confirm_task)
mcp.tool(reject_task)

# Inventory
mcp.tool(list_inventory)
mcp.tool(get_inventory)
mcp.tool(create_inventory)
mcp.tool(update_inventory)
mcp.tool(delete_inventory)

# Repositories
mcp.tool(list_repositories)
mcp.tool(get_repository)
mcp.tool(create_repository)
mcp.tool(update_repository)
mcp.tool(delete_repository)
mcp.tool(list_repository_branches)

# Environments
mcp.tool(list_environments)
mcp.tool(get_environment)
mcp.tool(create_environment)
mcp.tool(update_environment)
mcp.tool(delete_environment)

# Access Keys
mcp.tool(list_access_keys)
mcp.tool(get_access_key)
mcp.tool(create_access_key)
mcp.tool(delete_access_key)

# Schedules ★
mcp.tool(list_schedules)
mcp.tool(get_schedule)
mcp.tool(create_schedule)
mcp.tool(toggle_schedule)
mcp.tool(delete_schedule)
mcp.tool(validate_cron)

# Analytics ★
mcp.tool(get_project_analytics)
mcp.tool(get_task_trends)
mcp.tool(get_system_analytics)
mcp.tool(get_project_health)

# Runners ★
mcp.tool(list_runners)
mcp.tool(get_runner_status)
mcp.tool(toggle_runner)
mcp.tool(clear_runner_cache)

# Playbooks ★
mcp.tool(list_playbooks)
mcp.tool(get_playbook)
mcp.tool(sync_playbook)
mcp.tool(run_playbook)
mcp.tool(get_playbook_history)

# Audit & System ★
mcp.tool(get_audit_log)
mcp.tool(get_project_events)
mcp.tool(get_system_info)

# AI Analysis ★★
mcp.tool(analyze_task_failure)
mcp.tool(bulk_analyze_failures)


def main() -> None:
    transport = os.environ.get("MCP_TRANSPORT", "stdio")
    host = os.environ.get("MCP_HOST", "0.0.0.0")
    port = int(os.environ.get("MCP_PORT", "8500"))

    if transport == "http":
        mcp.run(transport="streamable-http", host=host, port=port, path="/mcp")
    else:
        mcp.run(transport="stdio")


if __name__ == "__main__":
    main()
