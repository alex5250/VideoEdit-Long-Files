


import os 
from moviepy.video.io.ffmpeg_tools import ffmpeg_extract_subclip

suffix='.mp4'

def crop(name,start,end,out):
	ffmpeg_extract_subclip(name, start, end, targetname=out)
pass

def convert_time(time):
	begin=time.split("-")[0]
	end=time.split("-")[1]
	if begin != None and end != None :
		minutes=begin.split(':')[0]
		seconds=begin.split(':')[1]
		result_begin=int(minutes)*60+int(seconds)
		minutes=end.split(':')[0]
		seconds=end.split(':')[1]
		result_end=int(minutes)*60+int(seconds)
		return result_begin,result_end

pass


human_script=input("Drop Human script  location:")
input_human_script_as_array=open(human_script,"r",encoding='UTF-8').readlines()

for line in input_human_script_as_array:
	filename=""
	final_name=''
	end=0
	start=0
	if list(line)[0]==";":
		filename=str(line.replace(";",""))
	string_length=len(line.split(","))
	if string_length > 1:
		final_name=line.split(",")[1]+suffix
		end,start=convert_time(line.split(",")[0])
	













