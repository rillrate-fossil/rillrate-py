from .rillrate import install, uninstall
from .rillrate import prime

from enum import Enum

class Activity(Enum):
    SUSPEND = 0
    AWAKE = 1
    DISCONNECTED = 2
    CONNECTED = 3
    ACTION = 4

class Action:
    value = None

    def __init__(self, value):
        self.value = value
