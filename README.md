# VideoEdit-Long-Files
Sometimes there is situation when large files such in 4K 60FPS approx. 50GB each needed to bee videoedited. If you will loading it in your editor like Resolve or Final Cut on your PC is hight likely that it will crashed. So as solution is this lua script for MPV and Rust app that will allow to do it. So the project contains from two components, first one is lua script for the MPV. It allows while watching the material in separate file config.dat make marking about interesting moments. Then this file will be uploaded to app that will allow to give comments and custom filename to it and process the croping pieces out of video by using ffmpeg.  


The lua script is based on : https://github.com/Arieleg/mpv-copyTime and it is highly modificated version of this script.  
Installation:  
1. Install MPV on your PC https://mpv.io/installation/  
2.  Put the script copyTime.lua in your scripts folder, usually in:   
     Windows: `"%APPDATA%\mpv\scripts".`  
     Linux and Mac:` "~/.config/mpv/scripts/".  `


