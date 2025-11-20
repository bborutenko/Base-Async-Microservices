from __future__ import annotations

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """
    Base application settings using pydantic-settings (Pydantic v2).

    You can override any field using environment variables, for example:
      - LOG_LEVEL=DEBUG
      - RELOAD=True
      - HOST=0.0.0.0
      - PORT=8000
      - KAFKA_BOOTSTRAP_SERVERS=kafka:9092

    The `.env` and `.env.local` files (if present) will also be loaded.
    """

    model_config = SettingsConfigDict(
        env_file=(".env.local", ".env"),
        env_file_encoding="utf-8",
        extra="ignore",
        case_sensitive=False,
    )

    log_level: str = Field(default="DEBUG")
    reload: bool = Field(default=True)
    host: str = Field(default="0.0.0.0")
    port: int = Field(default=8000)
    kafka_bootstrap_servers: str = Field(...)


settings = Settings()
