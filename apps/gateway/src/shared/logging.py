"""
Minimal logging utility based on logging.basicConfig.

Usage:
    from src.shared.simple_logging import setup_logging, get_logger

    setup_logging()  # once at startup
    logger = get_logger(__name__)
    logger.info("Service started")

Options:
    - Level resolution precedence:
        1) Explicit `level` argument
        2) Environment variable LOG_LEVEL or LOGLEVEL
        3) `debug=True` -> DEBUG
        4) Default -> INFO

    - Accepts standard level names ("DEBUG", "INFO", etc.) or numeric levels.
"""

from __future__ import annotations

import logging


def resolve_level(
    log_level: str,
) -> int:
    lvl = (log_level or "").strip().lower()
    if lvl == "debug":
        return logging.DEBUG
    if lvl == "info":
        return logging.INFO
    if lvl in ("warn", "warning"):
        return logging.WARNING
    if lvl == "error":
        return logging.ERROR
    if lvl in ("critical", "fatal"):
        return logging.CRITICAL
    return logging.DEBUG


def setup_logging(
    log_level: str,
    fmt: str | None = None,
    datefmt: str | None = None,
) -> None:
    """
    Initialize basic logging configuration. Safe to call more than once
    (basicConfig only applies on first call unless handlers are modified elsewhere).
    """
    resolved = resolve_level(log_level)

    if fmt is None:
        fmt = "%(asctime)s | %(levelname)s | %(name)s | %(message)s"
    if datefmt is None:
        datefmt = "%Y-%m-%d %H:%M:%S"

    logging.basicConfig(level=resolved, format=fmt, datefmt=datefmt)

    for name in ("uvicorn", "uvicorn.error", "uvicorn.access"):
        logging.getLogger(name).setLevel(resolved)


def get_logger(name: str | None = None) -> logging.Logger:
    """
    Get a logger. If name is None, returns the root logger.
    """
    return logging.getLogger(name if name else "")
