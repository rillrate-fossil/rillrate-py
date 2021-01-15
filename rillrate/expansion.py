import asyncio
from time import sleep
from threading import Thread

def gc_metrics():
    return True

def gc_stat_routine():
    import gc
    count0 = rillrate.Gauge("python.gc.count0")
    count1 = rillrate.Gauge("python.gc.count1")
    count2 = rillrate.Gauge("python.gc.count2")
    while True:
        (value0, value1, value2) = gc.get_count()
        count0.set(value0)
        count1.set(value1)
        count2.set(value2)

expansion_thread = None

def __expansion_routine():
    while True:
        print("updating metrics")
        sleep(1)

def __spawn_thread():
    global expansion_thread
    if expansion_thread == None:
        expansion_thread = Thread(target = __expansion_routine)
__spawn_thread()
