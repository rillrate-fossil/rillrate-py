#!/bin/env python3

import sys, os
sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
from time import sleep
from random import randint
import rillrate

rillrate.install()
value = rillrate.Gauge("main.gauge")
total = rillrate.Counter("main.total")
info = rillrate.Logger("main.info")

while True:
    total.inc(1)
    value.set(randint(1, 100))
    pause = randint(1, 20) / 100.0
    info.log("sleepping for " + str(pause) + "s")
    sleep(pause)
