from test_pkg import dummy_result


def test_assert() -> None:
    result: int = dummy_result() + 2
    assert result == 44
