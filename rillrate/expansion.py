from time import sleep
import gc
import threading
import rillrate

expansion_thread = None
metrics_pool = []

class GcMetrics:
    def install():
        global metrics_pool
        obj = GcMetrics()
        metrics_pool.append(obj)

    def __init__(self):
        # TODO: Use `Value` instead (like previous `Gauge`)
        self.count0 = rillrate.Pulse("python.metrics.gc.count0")
        self.count1 = rillrate.Pulse("python.metrics.gc.count1")
        self.count2 = rillrate.Pulse("python.metrics.gc.count2")

    def update(self):
        (value0, value1, value2) = gc.get_count()
        self.count0.push(value0)
        self.count1.push(value1)
        self.count2.push(value2)

class ThreadingMetrics:
    def install():
        global metrics_pool
        obj = ThreadingMetrics()
        metrics_pool.append(obj)

    def __init__(self):
        self.active_count = rillrate.Pulse("python.metrics.threading.active_count")

    def update(self):
         value = threading.active_count()
         self.active_count.push(value)

def __expansion_routine():
    global metrics_pool
    while True:
        for metric in metrics_pool:
            metric.update()
        sleep(1)

def __spawn_thread():
    global expansion_thread
    if expansion_thread == None:
        expansion_thread = threading.Thread(
            name = "rillrate.expansion",
            target = __expansion_routine,
            daemon = True,
        )
        expansion_thread.start()

__spawn_thread()
