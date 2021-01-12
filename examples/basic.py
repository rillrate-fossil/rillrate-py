#!/bin/env python3

import sys, os
sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
from time import sleep
from random import randint
import rillrate

rillrate.install()
g = rillrate.Gauge("main.gauge")
t = rillrate.Counter("main.total")
l = rillrate.Logger("main.info")

while True:
    t.inc(1)
    g.set(randint(1, 100))
    p = randint(1, 20) / 100.0
    l.log("sleepping for " + str(p) + "s")
    sleep(p)
