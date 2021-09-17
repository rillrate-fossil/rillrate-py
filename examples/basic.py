#!/usr/bin/env python3

import sys, os
# sys.path.append(os.path.join(sys.path[0],'..','target','debug'))
sys.path.append(os.getcwd())
from time import sleep
from random import randint
import rillrate
from rillrate import prime

rillrate.install()

paused = False
extra_ms = 5

alert = prime.Alert("example.dashboard.group-2.alert")

click = prime.Click("example.dashboard.group-1.click", label="Button")
def callback(activity, action):
    print("Click activity:", activity, "| action =", action)
    if action != None:
        print("Clicked!", action.value)
        click.apply()
        alert.notify("Bell")
        print("Click callback done")
click.sync_callback(callback)

inputt = prime.Input("example.dashboard.group-1.input", label="Enter the text")
def callback(activity, action):
    print("Input activity:", activity, "| action =", action)
    if action != None:
        print("Input:", action.value)
        inputt.apply()
        print("Input callback done")
inputt.sync_callback(callback)

selector = prime.Selector("example.dashboard.group-1.selector", label="Choose", options=["One", "Two", "Three"])
def callback(activity, action):
    print("Selector activity:", activity, "| action =", action)
    if action != None:
        print("Selected", action.value)
        selector.apply(action.value)
        print("Selector callback done")
selector.sync_callback(callback)

slider = prime.Slider("example.dashboard.group-1.slider", label="Extra ms", min=0, max=100, step=1)
slider.apply(extra_ms)
def callback(activity, action):
    print("Slider activity:", activity, "| action =", action)
    if action != None:
        print("Slider", action.value)
        global extra_ms
        extra_ms = action.value
        slider.apply(action.value)
        print("Slider callback done")
slider.sync_callback(callback)

switch = prime.Switch("example.dashboard.group-1.switch", label="Pause")
def callback(activity, action):
    print("Switch activity:", activity, "| action =", action)
    if action != None:
        print("Switch", action.value)
        global paused
        paused = action.value
        switch.apply(action.value)
        print("Switch callback done")
switch.sync_callback(callback)

counter = prime.Counter("example.dashboard.group-1.total")
gauge = prime.Gauge("example.dashboard.group-1.gauge", min=0, max=99, higher=True)
pulse = prime.Pulse("example.dashboard.group-1.pulse")
# logger = prime.Logger("example.dashboard.group-1.info")
hist = prime.Histogram("example.dashboard.group-1.histogram", levels=[100, 500, 1000])
board = prime.Board("example.dashboard.group-1.dict")
live_tail = prime.LiveTail("example.dashboard.group-2.live-tail")
table = prime.Table("example.dashboard.group-1.table", columns=[(0, "Col 1"), (1, "Col 2")])
table.add_row(0)
table.set_cell(0, 0, "pause")
table.add_row(1)
table.set_cell(1, 0, "random")

live_text = prime.LiveText("example.dashboard.group-2.text")
live_text.set("**Markdown** text")

print("Working...")
x = 0
while True:
    if not paused:
        x += 1
        live_tail.log_now("module", "DEBUG", "Iteration: " + str(x))
        counter.inc(1)
        gauge.set(randint(1, 100))
        pulse.push(randint(1, 100))
        hist.add(randint(40, 600))
        table.set_cell(1, 1, str(randint(1, 1000)))
        interval = randint(1, 20) / 100.0 + extra_ms / 100.0
        str_interval = '{:.2f}'.format(interval)
        board.set("interval", str_interval)
        table.set_cell(0, 1, str_interval)
        # logger.log("sleepping for " + str(interval) + "s")
    sleep(interval)
