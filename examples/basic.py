#!/usr/bin/env python3

import sys, os
# sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
sys.path.append(os.getcwd())
from time import sleep
from random import randint
import rillrate

rillrate.install()

# print(rillrate.__dict__)

click = rillrate.Click("example.dashboard.group-1.click", "Button")
def callback(activity, action):
    print("Click activity:", activity, "| action =", action)
    if action != None:
        print("Clicked!", action.value)
        click.clicked()
click.sync_callback(callback)

selector = rillrate.Selector("example.dashboard.group-1.selector", "Choose", ["One", "Two", "Three"])
def callback(activity, action):
    print("Selector activity:", activity, "| action =", action)
    if action != None:
        print("Selected", action.value)
        selector.select(action.value)
selector.sync_callback(callback)

slider = rillrate.Slider("example.dashboard.group-1.slider", "Slide", 0, 100, 1)
def callback(activity, action):
    print("Slider activity:", activity, "| action =", action)
    if action != None:
        print("Slider", action.value)
        slider.set(action.value)
slider.sync_callback(callback)

counter = rillrate.Counter("example.dashboard.group-1.total", True)
gauge = rillrate.Gauge("example.dashboard.group-1.gauge", 0, 100)
pulse = rillrate.Pulse("example.dashboard.group-1.pulse")
# logger = rillrate.Logger("example.dashboard.group-1.info")
hist = rillrate.Histogram("example.dashboard.group-1.histogram", [100, 500, 1000])
board = rillrate.Board("example.dashboard.group-1.dict")
table = rillrate.Table("example.dashboard.group-1.table", [(0, "Col 1"), (1, "Col 2")])
table.add_row(0)
table.set_cell(0, 0, "pause")
table.add_row(1)
table.set_cell(1, 0, "random")

print("Working...")
while True:
    counter.inc(1)
    gauge.set(randint(1, 100))
    pulse.push(randint(1, 100))
    hist.add(randint(40, 600))
    table.set_cell(1, 1, str(randint(1, 1000)))
    pause = randint(1, 20) / 100.0
    board.set("pause", str(pause))
    table.set_cell(0, 1, str(pause))
    # logger.log("sleepping for " + str(pause) + "s")
    sleep(pause)
