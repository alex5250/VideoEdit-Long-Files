[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)
# VideoEdit-Long-Files
Sometimes there is situation when large files such in 4K 60FPS approx. 50GB each needed to bee videoedited. If you will loading it in your editor like Resolve or Final Cut on your PC is hight likely that it will crashed. So as solution is this lua script for MPV and Rust app that will allow to do it. So the project contains from two components, first one is lua script for the MPV. It allows while watching the material in separate file config.dat make marking about interesting moments. Then this file will be uploaded to app that will allow to give comments and custom filename to it and process the croping pieces out of video by using ffmpeg.  


The lua script is based on : https://github.com/Arieleg/mpv-copyTime and it is highly modificated version of this script.  
Installation:  
1. Install MPV on your PC https://mpv.io/installation/  
2.  Put the script copyTime.lua in your scripts folder, usually in:   
     Windows: `"%APPDATA%\mpv\scripts".`  
     Linux and Mac:` "~/.config/mpv/scripts/".  `


Demonstration:

![Peek 2022-05-06 15-38](https://user-images.githubusercontent.com/20460747/167615770-74f706aa-6dc6-4429-a549-fcdeb03fe403.gif)

# One command install (console part only) (Linux only)
**READ ANY BASH FILES YOU RUN**
So clone it using curl and run it:
```bash
curl https://raw.githubusercontent.com/alex5250/VideoEdit-Long-Files/main/installer/install.sh | bash 
```
Demonstration:


https://user-images.githubusercontent.com/20460747/171739857-43394b8e-53a3-4fa3-bc89-93bc36063656.mp4

# New UI:
![image](https://user-images.githubusercontent.com/20460747/174448356-daf76864-e92f-4c11-990e-4f64714f40c7.png)

# Pure magic UI , on last release:
![1](https://user-images.githubusercontent.com/20460747/175174928-2f8b0540-bd6b-4157-bddf-e6c05fe55acf.png)
![2](https://user-images.githubusercontent.com/20460747/175174938-1c2ace37-f00d-4bbd-920a-73a191d42e68.png)
![3](https://user-images.githubusercontent.com/20460747/175174943-882d1f01-2b48-4f4b-bd5e-98f88be5fff0.png)


https://user-images.githubusercontent.com/20460747/175175113-227abbd0-3658-4f18-9613-b7de02ef6841.mp4



Todo:
-[ ] Make work on windows
-[ ] Allow users to skip editting part if config file is in dir
