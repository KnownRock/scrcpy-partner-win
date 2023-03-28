# from https://github.com/leng-yue/py-scrcpy-client

import functools
import socket
import struct
import threading
from time import sleep
import time


# Action
ACTION_DOWN = 0
ACTION_UP = 1
ACTION_MOVE = 2

# Type
TYPE_INJECT_KEYCODE = 0
TYPE_INJECT_TEXT = 1
TYPE_INJECT_TOUCH_EVENT = 2
TYPE_INJECT_SCROLL_EVENT = 3
TYPE_BACK_OR_SCREEN_ON = 4
TYPE_EXPAND_NOTIFICATION_PANEL = 5
TYPE_EXPAND_SETTINGS_PANEL = 6
TYPE_COLLAPSE_PANELS = 7
TYPE_GET_CLIPBOARD = 8
TYPE_SET_CLIPBOARD = 9
TYPE_SET_SCREEN_POWER_MODE = 10
TYPE_ROTATE_DEVICE = 11

# Screen power mode
POWER_MODE_OFF = 0
POWER_MODE_NORMAL = 2



def inject(control_type: int):
    """
    Inject control code, with this inject, we will be able to do unit test

    Args:
        control_type: event to send, TYPE_*
    """

    def wrapper(f):
        @functools.wraps(f)
        def inner(*args, **kwargs):
            package = struct.pack(">B", control_type) + f(*args, **kwargs)
            if args[0].parent.control_socket is not None:
                with args[0].parent.control_socket_lock:
                    args[0].parent.control_socket.send(package)
            return package

        return inner

    return wrapper


class ControlSender:
    def __init__(self, parent):
        self.parent = parent

    @inject(TYPE_INJECT_KEYCODE)
    def keycode(
        self, keycode: int, action: int = ACTION_DOWN, repeat: int = 0
    ) -> bytes:
        """
        Send keycode to device

        Args:
            keycode: KEYCODE_*
            action: ACTION_DOWN | ACTION_UP
            repeat: repeat count
        """
        return struct.pack(">Biii", action, keycode, repeat, 0)

    @inject(TYPE_INJECT_TEXT)
    def text(self, text: str) -> bytes:
        """
        Send text to device

        Args:
            text: text to send
        """

        buffer = text.encode("utf-8")
        return struct.pack(">i", len(buffer)) + buffer

    @inject(TYPE_INJECT_TOUCH_EVENT)
    def touch(
        self, x: int, y: int, action: int = ACTION_DOWN, touch_id: int = -1
    ) -> bytes:
        """
        Touch screen

        Args:
            x: horizontal position
            y: vertical position
            action: ACTION_DOWN | ACTION_UP | ACTION_MOVE
            touch_id: Default using virtual id -1, you can specify it to emulate multi finger touch
        """
        x, y = max(x, 0), max(y, 0)
        return struct.pack(
            ">BqiiHHHi",
            action,
            touch_id,
            int(x),
            int(y),
            int(self.parent.resolution[0]),
            int(self.parent.resolution[1]),
            0xFFFF,
            1,
        )

    @inject(TYPE_INJECT_SCROLL_EVENT)
    def scroll(self, x: int, y: int, h: int, v: int) -> bytes:
        """
        Scroll screen

        Args:
            x: horizontal position
            y: vertical position
            h: horizontal movement
            v: vertical movement
        """

        x, y = max(x, 0), max(y, 0)
        return struct.pack(
            ">iiHHii",
            int(x),
            int(y),
            int(self.parent.resolution[0]),
            int(self.parent.resolution[1]),
            int(h),
            int(v),
        )

    @inject(TYPE_BACK_OR_SCREEN_ON)
    def back_or_turn_screen_on(self, action: int = ACTION_DOWN) -> bytes:
        """
        If the screen is off, it is turned on only on ACTION_DOWN

        Args:
            action: ACTION_DOWN | ACTION_UP
        """
        return struct.pack(">B", action)

    @inject(TYPE_EXPAND_NOTIFICATION_PANEL)
    def expand_notification_panel(self) -> bytes:
        """
        Expand notification panel
        """
        return b""

    @inject(TYPE_EXPAND_SETTINGS_PANEL)
    def expand_settings_panel(self) -> bytes:
        """
        Expand settings panel
        """
        return b""

    @inject(TYPE_COLLAPSE_PANELS)
    def collapse_panels(self) -> bytes:
        """
        Collapse all panels
        """
        return b""

    def get_clipboard(self) -> str:
        """
        Get clipboard
        """
        # Since this function need socket response, we can't auto inject it any more
        s: socket.socket = self.parent.control_socket

        with self.parent.control_socket_lock:
            # Flush socket
            s.setblocking(False)
            while True:
                try:
                    s.recv(1024)
                except BlockingIOError:
                    break
            s.setblocking(True)

            # Read package
            package = struct.pack(">B", TYPE_GET_CLIPBOARD)
            s.send(package)
            (code,) = struct.unpack(">B", s.recv(1))
            assert code == 0
            (length,) = struct.unpack(">i", s.recv(4))

            return s.recv(length).decode("utf-8")

    @inject(TYPE_SET_CLIPBOARD)
    def set_clipboard(self, text: str, paste: bool = False) -> bytes:
        """
        Set clipboard

        Args:
            text: the string you want to set
            paste: paste now
        """
        buffer = text.encode("utf-8")
        return struct.pack(">?i", paste, len(buffer)) + buffer

    @inject(TYPE_SET_SCREEN_POWER_MODE)
    def set_screen_power_mode(self, mode: int = POWER_MODE_NORMAL) -> bytes:
        """
        Set screen power mode

        Args:
            mode: POWER_MODE_OFF | POWER_MODE_NORMAL
        """
        return struct.pack(">b", mode)

    @inject(TYPE_ROTATE_DEVICE)
    def rotate_device(self) -> bytes:
        """
        Rotate device
        """
        return b""

    def swipe(
        self,
        start_x: int,
        start_y: int,
        end_x: int,
        end_y: int,
        move_step_length: int = 5,
        move_steps_delay: float = 0.005,
    ) -> None:
        """
        Swipe on screen

        Args:
            start_x: start horizontal position
            start_y: start vertical position
            end_x: start horizontal position
            end_y: end vertical position
            move_step_length: length per step
            move_steps_delay: sleep seconds after each step
        :return:
        """

        self.touch(start_x, start_y, ACTION_DOWN)
        next_x = start_x
        next_y = start_y

        if end_x > self.parent.resolution[0]:
            end_x = self.parent.resolution[0]

        if end_y > self.parent.resolution[1]:
            end_y = self.parent.resolution[1]

        decrease_x = True if start_x > end_x else False
        decrease_y = True if start_y > end_y else False
        while True:
            if decrease_x:
                next_x -= move_step_length
                if next_x < end_x:
                    next_x = end_x
            else:
                next_x += move_step_length
                if next_x > end_x:
                    next_x = end_x

            if decrease_y:
                next_y -= move_step_length
                if next_y < end_y:
                    next_y = end_y
            else:
                next_y += move_step_length
                if next_y > end_y:
                    next_y = end_y

            self.touch(next_x, next_y, ACTION_MOVE)

            if next_x == end_x and next_y == end_y:
                self.touch(next_x, next_y, ACTION_UP)
                break
            sleep(move_steps_delay)

# only work on scrcpy 1.25

import os
from adbutils import AdbConnection, AdbDevice, AdbError, Network, adb
# device = adb.device_list()[0]

args = os.sys.argv[1:]

adb_id = args[0]

device = adb.device(adb_id)

jar_name = "scrcpy-server.jar"
server_file_path = os.path.join(
    os.path.abspath(os.path.dirname(__file__)), jar_name
)

print(server_file_path)

device_serial = device.serial

# os.system(f"adb -s {device_serial} push {server_file_path} /data/local/tmp/scrcpy-server.jar")

commands = [
    'CLASSPATH=/data/local/tmp/scrcpy-server.jar', 
    'app_process', 
    '/', 
    'com.genymobile.scrcpy.Server', 
    '1.25', 
    'tunnel_forward=true',
    # 'max_size=180',
    'max_fps=1',
    'power_on=false',
    ]
server_stream = device.shell(
  commands,
  stream=True,
)

server_stream.read(10)

sleep(0.1)

print('start')

video_socket = device.create_connection(
    Network.LOCAL_ABSTRACT, "scrcpy"
)

dummy_byte = video_socket.recv(1)
if not len(dummy_byte) or dummy_byte != b"\x00":
  raise ConnectionError("Did not receive Dummy Byte!")
print(dummy_byte)

control_socket = device.create_connection(
    Network.LOCAL_ABSTRACT, "scrcpy"
)

bits = video_socket.recv(64).decode("utf-8")
device_name = bits.rstrip("\x00")

print(device_name)
if not len(device_name):
    raise ConnectionError("Did not receive Device Name!")


res = video_socket.recv(4)
print(res)
resolution = struct.unpack(">HH", res)
video_socket.setblocking(False)

# TODO: handle rotation
real_size = os.popen('adb shell wm size').read().split(' ')[-1].strip()
print(real_size)

real_resolution = real_size.split('x')
real_resolution = [int(i) for i in real_resolution]

print(resolution)
from time import sleep
    
class MockClient:
  def __init__(self):
    self.control_socket = control_socket
    self.control_socket_lock = threading.Lock()
    self.resolution = resolution
    
    
client = MockClient()

controlSender = ControlSender(client)

#  def keycode(
#         self, keycode: int, action: int = ACTION_DOWN, repeat: int = 0
#     ) -> bytes:

def handle_keycode_args(keycode: str, action: str = str(ACTION_DOWN), repeat: str = str(0)):
    keycode = int(keycode)
    action = int(action)
    repeat = int(repeat)
    controlSender.keycode(keycode, action, repeat)

# def touch(
#     self, x: int, y: int, action: int = ACTION_DOWN, touch_id: int = -1
# ) -> bytes:

# use real resolution to control
def handle_touch_args(x: str, y: str, action: str = str(ACTION_DOWN), touch_id: str = str(-1)):
    x = int(x)
    y = int(y)
    
    x = int(x / real_resolution[0] * resolution[0])
    y = int(y / real_resolution[1] * resolution[1])
    
    action = int(action)
    touch_id = int(touch_id)
    controlSender.touch(x, y, action, touch_id)

# def text(self, text: str) -> bytes:
def handle_text_args(text: str):
    controlSender.text(text)
    
# def scroll(self, x: int, y: int, h: int, v: int) -> bytes:
def handle_scroll_args(x: str, y: str, h: str, v: str):
    x = int(x)
    y = int(y)
    h = int(h)
    v = int(v)
    controlSender.scroll(x, y, h, v)

# def set_screen_power_mode(self, mode: int = POWER_MODE_NORMAL) -> bytes:
def handle_set_screen_power_mode_args(mode: str = str(POWER_MODE_NORMAL)):
    mode = int(mode)
    controlSender.set_screen_power_mode(mode)
    
#    def swipe(
#     self,
#     start_x: int,
#     start_y: int,
#     end_x: int,
#     end_y: int,
#     move_step_length: int = 5,
#     move_steps_delay: float = 0.005,
# ) -> None: 
def handle_swipe_args(start_x: str, start_y: str, end_x: str, end_y: str, move_step_length: str = str(5), move_steps_delay: str = str(0.005)):
    start_x = int(start_x)
    start_y = int(start_y)
    end_x = int(end_x)
    end_y = int(end_y)
    move_step_length = int(move_step_length)
    move_steps_delay = float(move_steps_delay)
    
    controlSender.swipe(start_x, start_y, end_x, end_y, move_step_length, move_steps_delay)


f_dict = {
    "keycode": handle_keycode_args,
    "touch": handle_touch_args,
    "text": handle_text_args,
    "scroll": handle_scroll_args,
    "set_screen_power_mode": handle_set_screen_power_mode_args,
    "swipe": handle_swipe_args,
}
    

while True:
    inp = input()
    if len(inp) == 0:
        continue
    if inp == 'exit':
        break
    
    try:
        args = inp.split(' ')
        fuc = args[0]
        f_args = args[1:]
        
        print(fuc, f_args)
        
        # controlSender.__getattribute__(fuc)(*f_args)
        # print(f_args)
        f_dict[fuc](*f_args)
    except Exception as e:
        print(e)



# len_height = resolution[1] / 100

# controlSender.touch(resolution[0] / 2, 0, ACTION_DOWN)
# sleep(0.01)
# now = time.time()
# for i in range(95):
#   controlSender.touch(resolution[0] / 2, 0 + len_height * i, ACTION_MOVE)
# #   sleep(0.001)
  
# print(time.time() - now)
  
# controlSender.touch(resolution[0] / 2, 0 + len_height * 100, ACTION_UP)

# sleep(1)
