"""
Shared API routers for the gateway app.

This module exposes:
- `router` with a basic healthcheck endpoint.
- `include_routers(app)` helper to wire routers into the main FastAPI app.

Usage in your main application (apps/gateway/src/main.py):

    from fastapi import FastAPI
    from shared.routers import include_routers

    app = FastAPI(title="Gateway")
    include_routers(app)
"""

from fastapi import APIRouter

from .logging import get_logger

logger = get_logger()

router = APIRouter(tags=["Health"], prefix="/health")


@router.get(
    "",
    description="Basic liveness check endpoint. Returns 200 with a simple JSON payload.",
)
async def healthcheck() -> dict[str, str]:
    logger.info("Getting healthcheck request")
    return {"status": "ok"}
