# rillrate-py

Dynamic tracing system that tends to be real-time.

Python 3 bindings.

## How to use

Install the library:

```sh
pip install rillrate
```

Import it in your code and install a tracer:

```python
import rillrate
rillrate.install()
```

Add a metric and use methods to update it:

```python
gauge = rillrate.Gague("my.gauge");
gauge.set(123.0);
```

## Expansions
