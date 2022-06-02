#!/bin/bash
echo 'Install CutterDesktop MPV part (basic install mode)'
HOME="$(getent passwd $USER | awk -F ':' '{print $6}')"

declare -A osInfo;
osInfo[/etc/redhat-release]=yum
osInfo[/etc/arch-release]=pacman
osInfo[/etc/gentoo-release]=emerge
osInfo[/etc/SuSE-release]=zypp
osInfo[/etc/debian_version]=apt-get
osInfo[/etc/alpine-release]=apk

for f in ${!osInfo[@]}

do
    if [[ -f $f ]];then
        sudo ${osInfo[$f]} install dialog git mpv wget
        sudo ${osInfo[$f]} -Sy dialog git mpv  wget  -y 

    
 dialog --msgbox "Installation of app." 0 0

echo "lua script install"
mkdir "${HOME}/.config/mpv"
mkdir "${HOME}/.config/mpv/scripts/"
wget https://raw.githubusercontent.com/alex5250/VideoEdit-Long-Files/main/SplitVideo.lua -O /tmp/SplitVideo.lua
cp  /tmp/SplitVideo.lua "${HOME}/.config/mpv/scripts/SplitVideo.lua"

echo "binary file  install"

wget https://github.com/alex5250/VideoEdit-Long-Files/releases/download/v0.01-user-input/comment_input -O /tmp/bin_file
sudo cp /tmp/bin_file /usr/bin/cutter_desktop_comment_input
sudo chmod 755  /usr/bin/cutter_desktop_comment_input

    fi
done


