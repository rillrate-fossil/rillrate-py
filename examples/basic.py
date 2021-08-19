#!/usr/bin/env python3

import sys, os
# sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
sys.path.append(os.getcwd())
from time import sleep
from random import randint
import rillrate

rillrate.install()

# print(rillrate.__dict__)

paused = False
extra_ms = 5

click = rillrate.Click("example.dashboard.group-1.click", "Button")
def callback(activity, action):
    print("Click activity:", activity, "| action =", action)
    if action != None:
        print("Clicked!", action.value)
        click.apply()
click.sync_callback(callback)

selector = rillrate.Selector("example.dashboard.group-1.selector", "Choose", ["One", "Two", "Three"])
def callback(activity, action):
    print("Selector activity:", activity, "| action =", action)
    if action != None:
        print("Selected", action.value)
        selector.apply(action.value)
selector.sync_callback(callback)

slider = rillrate.Slider("example.dashboard.group-1.slider", "Extra ms", 0, 100, 1)
slider.apply(extra_ms)
def callback(activity, action):
    print("Slider activity:", activity, "| action =", action)
    if action != None:
        print("Slider", action.value)
        global extra_ms
        extra_ms = action.value
        slider.apply(action.value)
slider.sync_callback(callback)

switch = rillrate.Switch("example.dashboard.group-1.switch", "Pause")
def callback(activity, action):
    print("Switch activity:", activity, "| action =", action)
    if action != None:
        print("Switch", action.value)
        global paused
        paused = action.value
        switch.apply(action.value)
switch.sync_callback(callback)

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
    if not paused:
        counter.inc(1)
        gauge.set(randint(1, 100))
        pulse.push(randint(1, 100))
        hist.add(randint(40, 600))
        table.set_cell(1, 1, str(randint(1, 1000)))
        interval = randint(1, 20) / 100.0 + extra_ms / 100.0
        board.set("interval", str(interval))
        table.set_cell(0, 1, str(interval))
        # logger.log("sleepping for " + str(interval) + "s")
    sleep(interval)
