
*option 1, single app*
1 python app:
for viewing, detection, recording, ptz

Another app for detection and recording only...?? How to debug?
Better to make one app that does all, with option to show a UI.

*option 2, split*
 - dumb recorder that just dump's all video to disk, rotating, 1 per minute per device.
   : Moonfire NVR (run on pi?)
 - smart analyser and viewer, using cv2 and YOLO, records events to a db.
   Moves specific recorded files to longer storage on event trigger.


*1 android app*
for viewing and ptz
- can view live
- can move ptz?
- can list triggers and view related recordings


================

rust trial, then use iced as stream viewer?

cargo add ultralytics-inference
ultralytics-inference = { version = "0.0.50", features = ["cuda"] } 


=================

**analysing frames**

python easy, use cv2.
in rust?


