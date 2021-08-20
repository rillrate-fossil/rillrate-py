# rillrate-py

[![PyPI][pypi-badge]][pypi-url]

[pypi-badge]: https://badge.fury.io/py/rillrate.svg
[pypi-url]: https://pypi.org/project/rillrate

Real-time UI for bots.

<img align="center" width="400px" style="margin-left: 20px;" src="https://rillrate.com/images/dashboard.png" />

Python 3 bindings for [**RillRate**][rillrate].

[rillrate]: https://github.com/rillrate/rillrate

## How to use

Install the library:

```sh
pip install rillrate
```

Import it in your code and install a tracer:

```python
import rillrate
rillrate.install("my-app")

board = rillrate.Board("package-1.dashboard-1.group-1.board");
board.set("Key", "Value");
```

## Expansions

You can also monitor internal Python stats.
