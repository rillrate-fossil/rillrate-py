import asyncio

def gc_metrics():
    global gc_stat_task
    gc_stat_task = asyncio.create_task(gc_stat_routine())

async def gc_stat_routine():
    import gc
    count0 = rillrate.Gauge("python.gc.count0")
    count1 = rillrate.Gauge("python.gc.count1")
    count2 = rillrate.Gauge("python.gc.count2")
    while True:
        (value0, value1, value2) = gc.get_count()
        count0.set(value0)
        count1.set(value1)
        count2.set(value2)
        await asyncio.sleep(1)
