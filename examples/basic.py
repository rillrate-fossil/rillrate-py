#!/bin/env python3

import sys, os
sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
from time import sleep
from random import randint
import rillrate

rillrate.install()
g = rillrate.GaugeProvider(["main", "gauge"])
t = rillrate.CounterProvider(["main", "total"])

while True:
    t.inc(1)
    g.set(randint(1, 100))
    sleep(0.1)
