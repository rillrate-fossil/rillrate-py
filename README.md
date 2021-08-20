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

### Board

<img src="https://docs.rillrate.com/images/tracers/flows/board.gif" width="200px" align="left">

Board of keys and values.

```python
board = rillrate.Board("package-1.dashboard-1.group-1.pulse");
board.set("Key", "Value");
```

### Counter

<img src="https://docs.rillrate.com/images/tracers/flows/counter.gif" width="200px" align="left">

Integer counter for something.

```python
counter = rillrate.Counter("package-1.dashboard-1.group-1.counter");
counter.inc(123);
```

### Gauge

<img src="https://docs.rillrate.com/images/tracers/flows/gauge.gif" width="200px" align="left">

Shows value in a range.

```python
gauge = rillrate.Gauge("package-1.dashboard-1.group-1.gauge");
gauge.set(10);
```

### Pulse

<img src="https://docs.rillrate.com/images/tracers/flows/pulse.gif" width="200px" align="left">

Renders a chart of the latest values slice.

```python
pulse = rillrate.Pulse("package-1.dashboard-1.group-1.pulse");
pulse.push(50.2);
```

### Table

<img src="https://docs.rillrate.com/images/tracers/flows/table.gif" width="200px" align="left">

Table with rows and cols.

```python
table = rillrate.Table("package-1.dashboard-1.group-1.table");
table.set(1, 2, "cell");
```


## Controls

### Click

<img src="https://docs.rillrate.com/images/tracers/controls/click.gif" height="100px" align="left">

A clickable button.

```python
click = rillrate.Click("package-1.dashboard-1.group-1.click");
click.clicked()
```

### Selector

<img src="https://docs.rillrate.com/images/tracers/controls/selector.gif" width="200px" align="left">

Selector of value from a set.

```python
selector = rillrate.Selector("package-1.dashboard-1.group-1.selector");
selector.select("Value")
```

### Slider

<img src="https://docs.rillrate.com/images/tracers/controls/slider.gif" width="200px" align="left">

Slider to choose value in a range.

```python
slider = rillrate.Slider("package-1.dashboard-1.group-1.slider");
slider.set(12.0)
```

### Switch

<img src="https://docs.rillrate.com/images/tracers/controls/switch.gif" height="100px" align="left">

Switch is a toggleable button with a boolean value.

```python
switch = rillrate.Switch("package-1.dashboard-1.group-1.switch");
switch.turn(True)
```

## Expansions
