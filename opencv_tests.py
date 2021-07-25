import cv2
import numpy as np
import paho.mqtt.client as mqtt
import json

client = mqtt.Client()

client.connect("localhost", 1883, 60)

# Blocking call that processes network traffic, dispatches callbacks and
# handles reconnecting.
# Other loop*() functions are available that give a threaded interface and a
# manual interface.
client.loop_start()


cam = cv2.VideoCapture(0)

cv2.namedWindow("test")

img_counter = 0

# Setup SimpleBlobDetector parameters.
params = cv2.SimpleBlobDetector_Params()

# Change thresholds
params.minThreshold = 10
params.maxThreshold = 200


# Filter by Area.
params.filterByArea = True
params.minArea = 200
params.maxArea = 200000
# Filter by Circularity
#params.filterByCircularity = True
#params.minCircularity = 0.1

# Filter by Convexity
#params.filterByConvexity = True
#params.minConvexity = 0.87

params.filterByCircularity = False
params.filterByInertia = False
params.filterByConvexity = False

params.filterByColor = True # Toggle only this line
params.blobColor = 255

#orangeLower = (0,30,70)  #100,130,50 #BGR not RGB!
#orangeHigher = (70,110,255) #200,200,130

orangeLower = (30,50, 10)  #100,130,50 #BGR not RGB!
orangeHigher = (90,255, 255) #200,200,130


# Create a detector with the parameters
ver = (cv2.__version__).split('.')
if int(ver[0]) < 3 :
    detector = cv2.SimpleBlobDetector(params)
else : 
    detector = cv2.SimpleBlobDetector_create(params)
cnt = 0
while True:
    ret, frame = cam.read()
    hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
    if not ret:
        print("failed to grab frame")
        break
    
    cnt = cnt+1
    
    if cnt > 10:
        mask = cv2.inRange(hsv, orangeLower, orangeHigher)
        
        mask = cv2.erode(mask, None, iterations=0)
        mask = cv2.dilate(mask, None, iterations=0)
        masked_frame = cv2.bitwise_and(hsv,hsv,mask = mask)
        
        detector.empty() 
        cv2.imshow("test3", mask)
        keypoints = detector.detect(mask)
        x = 0
        y = 0
        if len(keypoints) > 0:
            print(keypoints[0].pt)
            x = keypoints[0].pt[0]
            y = keypoints[0].pt[1]
        
        blob = dict()
        blob["x"] = round(x)
        blob["y"] = round(y)
        payload = json.dumps(blob)
        client.publish("camera/blob", payload)
        im_with_keypoints = cv2.drawKeypoints(frame, keypoints, np.array([]), (0,0,255), cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
        cv2.imshow("test", im_with_keypoints)
        cv2.imshow("test2", masked_frame)
        

        
    k = cv2.waitKey(1)
    if k%256 == 27:
        # ESC pressed
        print("Escape hit, closing...")
        break
    elif k%256 == 32:
        # SPACE pressed
        img_name = "opencv_frame_{}.png".format(img_counter)
        cv2.imwrite(img_name, frame)
        print("{} written!".format(img_name))
        img_counter += 1

cam.release()

cv2.destroyAllWindows()

#350, 235
