import asyncio
from time import sleep
from threading import Thread
import rillrate

# TODO: Move to a separate module

import gc

class GcMetrics:
    def __init__(self):
        self.count0 = rillrate.Gauge("python.gc.count0")
        self.count1 = rillrate.Gauge("python.gc.count1")
        self.count2 = rillrate.Gauge("python.gc.count2")

    def update(self):
        print("updating gc metrics")
        (value0, value1, value2) = gc.get_count()
        self.count0.set(value0)
        self.count1.set(value1)
        self.count2.set(value2)

expansion_thread = None
metrics_pool = []

def gc_metrics():
    global metrics_pool
    obj = GcMetrics()
    metrics_pool.append(obj)

def __expansion_routine():
    global metrics_pool
    while True:
        print("updating metrics")
        for metric in metrics_pool:
            metric.update()
        sleep(1)

def __spawn_thread():
    global expansion_thread
    if expansion_thread == None:
        expansion_thread = Thread(
            name = "rillrate.expansion",
            target = __expansion_routine,
            daemon = True,
        )
        expansion_thread.start()

__spawn_thread()
