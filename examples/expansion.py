#!/bin/env python3

import sys, os
# sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
sys.path.append(os.getcwd())
from time import sleep
from random import randint
import rillrate
import rillrate.expansion

while True:
    rillrate.expansion.gc_metrics()
    sleep(1)
