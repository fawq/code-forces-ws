from test_pkg_rust_python._core import get_fine_number


def test_assert() -> None:
    result: int = get_fine_number() + 2
    assert result == 6
