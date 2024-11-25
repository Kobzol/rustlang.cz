# Rust Prague meetup
This guide documents some stuff that should be useful for organizing the Rust Prague meetup.
The guide assumes that the meetup is happening at MFF (Malostranské nám. 25, Malá Strana, Praha 1).

## Orientation
At the bottom floor of MFF, there should be orientation signs posted up, to let people know where should they go. This is usually done by Martin D.

## Audio/video setup
There is an A/V control panel at the main desk. You can use it to turn the lights on or off, change the volume of the microphone, and select the source of what is presented on the wall.

First, start up the computer at the main desk. Its monitor needs to be raised up through a button on the desk. There should also be login information written on the desk, use that to log into the computer.

### Video (presenting)
Slides can be presented in three ways:
1. From the desk computer. In this case, switch the source on the A/V panel to the desk computer.
2. From an external laptop connected through an HDMI extender (usually provided by Martin D.). The extender should be connected to the desk computer, and the source on the A/V panel should be set to the desk computer. This allows both presenting and recording with an external laptop.
3. From an external laptop connected through HDMI. In this case, switch the source on the A/V panel to HDMI. **This does not allow recording the presentation!**

### Audio
There are two types of microphones (hand and ear) in a drawer in the main desk (keys provided by Martin D.). I'd suggest using the ear microphone. It needs to be turned on. Its volume can be modified via the A/V panel.

During Q/A sessions, it is nice to repeat the asked questions, so that they can be heard on the recording/stream.

### Recording the meetup
Recording is performed through [OBS](https://obsproject.com/), which is preinstalled on the desk computer. You can either only record or both record and stream (the former is easier, of course).

The OBS on the computer needs to be configured. Note that the configuration is transient and will disappear after restart of the computer (perhaps even after restart of OBS, so don't turn it off after configuring it).

1. Create a directory for the meetup at Desktop.
2. Set the recording directory to that directory (in `Settings -> Output -> Recording -> Recording path`). **This is very important, because the default recording directory has only very little disk space!**
3. Set `Recording Format` to MPEG-4 or MKV.
4. (If you also want to stream) Set `Recording Quality` to `Same as stream`.
5. (If you also want to stream) Open `Stream`, select `YouTube - RTMPS` service and `Use Stream Key`. Then you need to fill in the key that you get on YouTube (`studio.youtube.com` -> `Create` -> `Go live`). 
6. In the bottom left corner of the main OBS screen (`Scenes`), select the scene that shows the presented laptop/PC screen, overlaid with a camera shot of the presenter in the top right corner.
7. (If you also want to stream) Add a scene with some static background image, and switch to it before starting the stream, to avoid showing background audio/video on the stream before the meetup really starts. 
8. Turn off the desktop audio output of the presenting PC (unless you need to record its audio).
9. Check that the used microphone is producing audio signal in OBS.
10. Click on `Start Recording` in the bottom right corner. If you want to stream, click on `Start streaming` instead, and then also on `Start recording`, if OBS is not already recording.
11. Once the recorded part of the meetup ends, click on `Stop Recording`.
12. **Copy the recorded mp4/mkv file from the directory in Desktop to a USB flash drive, so that you can upload it later!** It will be removed after restart of the PC! The desk computer has several USB ports on the left side (from where you sit). If some of them don't work, try a different one.

### Wrapping up
After the meetup ends, make sure to:

- Turn off the microphone.
- Copy the recording to a USB flash drive.
- Turn off the desk computer.
- Turn off the lights through the A/V panel.
