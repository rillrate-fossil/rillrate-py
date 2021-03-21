#!/usr/bin/env python3

import sys, os
# sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
sys.path.append(os.getcwd())
from time import sleep
from random import randint
import rillrate

rillrate.install()
counter = rillrate.Counter("main.total")
gauge = rillrate.Gauge("main.gauge", 0, 100)
pulse = rillrate.Pulse("main.pulse")
logger = rillrate.Logger("main.info")
hist = rillrate.Histogram("main.histogram", [100, 500, 1000])
rrdict = rillrate.Dict("main.dict")
table = rillrate.Table("main.table", [(0, "Col 1"), (1, "Col 2")])
table.add_row(0)
table.set_cell(0, 0, "pause")
table.add_row(1)
table.set_cell(1, 0, "random")

while True:
    counter.inc(1)
    gauge.set(randint(1, 100))
    pulse.set(randint(1, 100))
    hist.add(randint(40, 600))
    table.set_cell(1, 1, str(randint(1, 1000)))
    pause = randint(1, 20) / 100.0
    rrdict.set("pause", str(pause))
    table.set_cell(0, 1, str(pause))
    logger.log("sleepping for " + str(pause) + "s")
    sleep(pause)
