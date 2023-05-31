import cv2, time, pandas, os
from datetime import datetime

DEBUG = False
USE_MAIN = True

#in order to deal with the over sensitivity to light sources, light splash
#and the sky, we should look inside the bounding boxes, and say, if the
#brightness or whiteness is over a certain threshold that it should not count
#as motion. if that doesnt work well we can also say that if we are already
#recording then if the whiteness is over a certain threshold, ignore the motion

# we need a framebuffer
# with a method push_frame
# and a method flush_buffer
# and use that method in place of all output_file.write
# the framebuffer needs to keep track of
#   the number of frames in the buffer
#   the number of frames before a frame is popped from the front
#   
# flush buffer needs to return the output_file it was using
#
# it needs to check at each push if the buffer is over the limit
# and pop from the front to keep it at the limit

class FrameBuffer:
    def __init__(self):
        self.max_size = 25 * 60
        self.length = 0
        self.buffer = []
        self.flush_condition = False

    def push(self, frame, output_file):
        global NUMBER_OF_BOXES
        if NUMBER_OF_BOXES > 1:
            return output_file
        if self.length + 1  > self.max_size:
            self.buffer.pop(0)
            time = datetime.utcnow()
            frame = add_timestamp_to_frame(frame, time)
            self.buffer.append(frame)
        else:
            time = datetime.utcnow()
            frame = add_timestamp_to_frame(frame, time)
            self.buffer.append(frame)
            self.length += 1
        
        if self.flush_condition:
            if self.length != 0:
                count = 0
                while count < 2 and self.length > 0:
                    if output_file != None:
                        output_file.write(self.buffer.pop(0))
                    else:
                        print("output file was none during flush condition")
                    count += 1
                    self.length -= 1
                if DEBUG:
                    print("buffer was "+str(self.length)+" when flushed in push")
                if self.length != 0:
                    self.flush_condition = True
                else:
                    self.flush_condition = False

            else:
                self.flush_condition = False
        return output_file

    
    def mini_flush(self, output_file):
        if NUMBER_OF_BOXES > 1:
            return output_file

        if output_file == None:
            output_file = start_output_file()

        if self.length != 0:
            count = 0
            while count < 2 and self.length > 0:
                if output_file != None:
                    output_file.write(self.buffer.pop(0))
                else:
                    print("output file was none in mini_flush")
                count += 1
                self.length -= 1
            if self.length != 0:
                self.flush_condition = True
            else:
                self.flush_condition = False

        else:
            self.flush_condition = False
        return output_file

    def flush_all(self, output_file):
        while self.length != 0:
            if output_file != None:
                output_file.write(self.buffer.pop(0))
            else:
                print("output file was none during flush_all")
            self.length -= 1
            if DEBUG:
                print("buffer is "+str(self.length))
        return output_file



USING = None
GREEN = "USB20FD1"
RED = "USB20FD"
MEDIA = "/media/oldie/"
DEVICES = {
    "USB20FD1": "/dev/sdc1",
    "USB20FD": "/dev/sdb1"
}

def check_storage(output_file):
    global USING
    global GREEN
    global RED
    global MEDIA
    global DEVICES
    for dev in DEVICES.keys():
        if os.path.isdir(MEDIA+GREEN) and USING != GREEN:
            print("switching to "+GREEN)
            if USING == None:
                USING = GREEN
                continue
            if output_file != None:
                output_file.release()
                output_file = None
                time.sleep(1)
            os.system("umount "+DEVICES[RED])
            while os.path.isdir(MEDIA+RED):
                time.sleep(1)

            USING = GREEN
        elif os.path.isdir(MEDIA+RED) and USING != RED:
            print("switching to "+RED)
            if USING == None:
                USING = RED
                continue
            if output_file != None:
                output_file.release()
                output_file = None
                time.sleep(1)
            os.system("umount "+DEVICES[GREEN])
            while os.path.isdir(MEDIA+GREEN):
                time.sleep(1)
            USING = RED
    return output_file

def log_bat():
    os.system("acpi | ts >> bat.log")

def log_ram():
    os.system("free -m | grep Mem | ts >> mem.log")

def video_did_not_open():
    global VIDEO
    return not VIDEO.isOpened()

def start_camera():
    return cv2.VideoCapture()

def format_time():
    local_time = datetime.now()
    if local_time.hour > 12:
        hour = local_time.hour - 12
        time_only = left_pad(hour)+left_pad(local_time.minute)+left_pad(local_time.second)+" PM PST"
    elif local_time.hour == 0:
        hour = 12
        time_only = left_pad(hour)+left_pad(local_time.minute)+left_pad(local_time.second)+" AM PST"
    else:
        time_only = left_pad(local_time.hour)+left_pad(local_time.minute)+left_pad(local_time.second)+" AM PST"
    return left_pad(local_time.month)+"-"+left_pad(local_time.day)+"-"+str(local_time.year)+" "+time_only
    


def start_output_file():
    global MEDIA
    global USING
    global VIDEO
    FRAME_WIDTH = int(VIDEO.get(3))
    FRAME_HEIGHT = int(VIDEO.get(4))
    local_formatted_time = format_time()
    file_name = MEDIA+USING+"/"+local_formatted_time+".mp4"
    print("started_recording to "+file_name)
    return cv2.VideoWriter(file_name, cv2.VideoWriter_fourcc('M','P','4','V'), 10, (FRAME_WIDTH, FRAME_HEIGHT))

def motion_times_is_not_empty(motion_times):
    return motion_times != []

def motion_just_started(motion_history):
    if motion_history[-1] == None:
        return False
    if motion_history[-2] == None and motion_history[-1] == 1 or motion_history[-1] == 1 and motion_history[-2] == 0:
            return True

def motion_just_ended(motion_history):
    return motion_history[-1] == 0 and motion_history[-2] == 1

def keep_only_the_last_two_motions(motion_history):
    return motion_history[-2:]

def add_motion_to_history(motion):
    motion_history.append(motion)

def add_bounding_boxes_to_contours(countours, frame, motion):
    global NUMBER_OF_BOXES
    NUMBER_OF_BOXES = 0
    for contour in contours:
        if cv2.contourArea(contour) < 10000:
            continue
        motion = 1
        (x, y, w, h) = cv2.boundingRect(contour)
        cv2.rectangle(frame, (x, y), (x + w, y + h), (0, 255, 0), 3)
        NUMBER_OF_BOXES += 1
    return frame, motion

def find_contours(threshold_frame):
    contours, _ = cv2.findContours(threshold_frame.copy(), cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    return contours

def threshold_before_activation(difference_frame):
    global THRESHOLD
    threshold_frame = cv2.threshold(difference_frame, THRESHOLD, 255, cv2.THRESH_BINARY)[1]
    threshold_frame = cv2.dilate(threshold_frame, None, iterations = 2)
    return threshold_frame

def difference_between_current_and_motionless(frame, motionless_frame):
    return cv2.absdiff(motionless_frame, gray_frame) 

def blur_grayscale_frame(frame):
    gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    gray_frame = cv2.GaussianBlur(gray_frame, (21, 21), 0)
    return gray_frame

def display_frames(difference_frame, threshold_frame, frame):
    cv2.imshow("Difference Frame", difference_frame)
    cv2.imshow("Threshold Frame", threshold_frame)
    cv2.imshow("Color Frame", frame)

def display_frame(frame):
    cv2.imshow("Color Frame", frame)

def build_movement_time_table():
    for i in range(0, len(motion_times), 2):
        table = table.concat({"Start":motion_times[i], "End":motion_times[i + 1]}, ignore_index = True)

def save_movement_time_table():
    table.to_csv('motion_times of Movements.csv')

def motion_occured_less_than_60_seconds_ago(motion_times):
    if motion_times[-1] == None:
        return False
    else:
        time = datetime.utcnow()
        difference = time - motion_times[-1]
        if difference.seconds < 60:
            return True
        else:
            return False

def wait_a_sec_for_quit_key():
    key = cv2.waitKey(1)
    return key == ord("q")

def wait_a_sec_for_full_speed():
    key = cv2.waitKey(1)
    return key == ord(" ")

def current_frame_has_motion(motion):
    return motion == 1

def add_timestamp_to_frame(frame, time):
    font = cv2.FONT_HERSHEY_PLAIN
    formatted_time = format_time()
    cv2.putText(frame, formatted_time, (20, 40), font, 2, (0, 0, 255), 2, cv2.LINE_AA)
    return frame

def write_stamped_frame(frame, output_file):
    global FRAME_BUFFER
    time = datetime.utcnow()
    frame = add_timestamp_to_frame(frame, time)
    if output_file != None:
        output_file = FRAME_BUFFER.push(frame, output_file)
        output_file = FRAME_BUFFER.mini_flush(output_file)
    else:
        check_storage()
        output_file = start_output_file()
        output_file = FRAME_BUFFER.push(frame, output_file)
        output_file = FRAME_BUFFER.mini_flush(output_file)

    return output_file

def left_pad(number):
    if number < 10:
        return "0"+str(number)
    else:
        return str(number)


motionless_frame = None

motion_history = [None, None]

motion_times = []

NUMBER_OF_BOXES = 0
THRESHOLD = 15
threshold_minimum = 15

table = pandas.DataFrame(columns = ["Start", "End"])
time.sleep(5)
VIDEO = start_camera()
last_reset_at = datetime.utcnow()
started_recording_at = None
FRAME_BUFFER = FrameBuffer()
show_frame = False

VIDEO.open("/dev/v4l/by-id/usb-Sonix_Technology_Co.__Ltd._USB_2.0_Camera-video-index0")
output_file = None

if video_did_not_open(): 
  print("Unable to read camera feed, switching to main")
  if USE_MAIN:
    VIDEO.open("/dev/v4l/by-id/usb-Generic_USB2.0_HD_UVC_WebCam_0x0001-video-index0")
  else:
    exit()

output_file = check_storage(output_file)
while USING == None:
    print("storage not detected")
    output_file = check_storage(output_file)

stop_recording = False

speed_key = False

last_logged = None

while not stop_recording:
    output_file = check_storage(output_file)
    
    #time_since_reset = datetime.utcnow() - last_reset_at
    #if time_since_reset.seconds > 1500:
    #    last_reset_at = datetime.utcnow()
    #    threshold_minimum = 15


    if last_logged != None:
        time_since_last_log = datetime.utcnow() - last_logged

    if speed_key:
        show_frame = True

    if last_logged == None or time_since_last_log.seconds > 300:
        log_bat()
        log_ram()
        show_frame = True
        #if output_file == None:
        #    if THRESHOLD > threshold_minimum:
        #        THRESHOLD -= 1
        last_logged = datetime.utcnow()
        if output_file != None:
            print("stopped recording")
            output_file.release()
            if output_file.isOpened():
                print("output still open")
            output_file = None
    check, frame = VIDEO.read()
    frame = add_timestamp_to_frame(frame, time)
    if output_file == None:
        output_file = start_output_file()
    output_file.write(frame)

    #if output_file != None:
    #    if started_recording_at == None:
    #        started_recording_at = datetime.utcnow()


    #if motion_times_is_not_empty(motion_times):
    #    if motion_occured_less_than_60_seconds_ago(motion_times):
    #        output_file = write_stamped_frame(frame, output_file)
    #    else:
    #        motion_history = [None, None]
    #        motion_times = []
    #        output_file = FRAME_BUFFER.push(frame, output_file)
    #        started_recording_at = None
    #        if output_file != None:
    #            if FRAME_BUFFER.length != 0:
    #                output_file = FRAME_BUFFER.flush_all(output_file)
    #            output_file.release()
    #            print("stopped_recording")
    #            output_file = None
    #else:
    #    output_file = FRAME_BUFFER.push(frame, output_file)

    #motion = 0

    #gray_frame = blur_grayscale_frame(frame)
    
    #if motionless_frame is None:
    #    motionless_frame = gray_frame
    #    continue
    
    #difference_frame = difference_between_current_and_motionless(frame, motionless_frame)
    
    #threshold_frame = threshold_before_activation(difference_frame)
    
    #contours = find_contours(threshold_frame)
    
    #frame, motion = add_bounding_boxes_to_contours(contours, frame, motion)
    
    #if motion == 1:
    #    if started_recording_at != None: 
    #        duration = datetime.utcnow() - started_recording_at
    #        if duration.seconds > 60:
    #            THRESHOLD += 1
    #            threshold_minimum += 1
    #            print("increased threshold_minimum: "+str(threshold_minimum))

    #    motion_times.append(datetime.utcnow())
    #    if DEBUG:
    #        print("seen something move, threshold: "+str(THRESHOLD))
    #        print("number_of_boxes: "+str(NUMBER_OF_BOXES))

    #        if NUMBER_OF_BOXES > 1:
    #            THRESHOLD += 1
    #            continue

                        
    #add_motion_to_history(motion)
    
    #motion_history = keep_only_the_last_two_motions(motion_history)

    #if motion_just_started(motion_history):
    #    output_file = write_stamped_frame(frame, output_file)
    #elif motion_just_ended(motion_history):
    #    output_file = write_stamped_frame(frame, output_file)
    quit_key = wait_a_sec_for_quit_key()
    speed_key = wait_a_sec_for_full_speed()
    if quit_key:
        #if current_frame_has_motion(motion):
        #    output_file = write_stamped_frame(frame, output_file)
        VIDEO.release()
        if output_file != None:
            print("stopped recording")
            output_file.release()
            if output_file.isOpened():
                print("output still open")
        cv2.destroyAllWindows()
        stop_recording = True
    if show_frame == True:
        #display_frames(difference_frame, threshold_frame, frame)
        display_frame(frame)

        show_frame = False


    


