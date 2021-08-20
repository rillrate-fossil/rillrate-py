# rillrate-py

[![PyPI][pypi-badge]][pypi-url]

[pypi-badge]: https://badge.fury.io/py/rillrate.svg
[pypi-url]: https://pypi.org/project/rillrate

Real-time UI for bots.

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
```

## Metrics

Add metrics allow to to monitor some properties of your bot.

### Counter

<img src="https://cdn.rillrate.com/github/rillrate/tracers/counter.png" width="200" align="left">

Integer counter for something.

```python
gauge = rillrate.Pulse("package-1.dashboard-1.group-1.counter");
gauge.inc(123);
```

### Pulse

<img src="https://cdn.rillrate.com/github/rillrate/tracers/pulse.gif" width="200" align="left">

Renders a chart of the latest values slice.

```python
gauge = rillrate.Pulse("package-1.dashboard-1.group-1.pulse");
gauge.push(50.2);
```

### Board

<img src="https://cdn.rillrate.com/github/rillrate/tracers/board.png" width="200" align="left">

Board of keys and values.

```python
gauge = rillrate.Board("package-1.dashboard-1.group-1.pulse");
gauge.set("Key", "Value");
```

## Controls

## Expansions
