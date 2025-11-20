from uuid import UUID, uuid4


def get_correlation_id() -> UUID:
    return uuid4()
