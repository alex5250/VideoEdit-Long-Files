require 'mp'
require 'mp.msg'



runned_times=0




local function divmod(a, b)
    return a / b, a % b
end



local function write_to_file(time,way)
		file = io.open("config.dat", "a")
	    file:write(string.format("%s_%s=%s \n",runned_times,way, time))
	    file:close()

	-- body
end

local function comment_append(comment)
    file = io.open("config.dat", "a")
    file:write(string.format("%s_%s=%s \n",runned_times,"comment", comment))
    file:close()
-- body
end
local function make_out_file_begin(time,way)

    if runned_times == 0
		then
			mp.osd_message(string.format("new .dat file  %s", video_path))
			local video_path = mp.get_property("path")
			file = io.open("config.dat", "a")
		    file:write(string.format("[ %s ] \n", video_path))
		    file:close()

		    write_to_file(time,way)

	else

		write_to_file(time,way)

    end

	-- body
end

local function make_out_file_end(time,way)
    	write_to_file(time,way)
end

local function get_time()
	local video_path = mp.get_property("path")
    local time_pos = mp.get_property_number("time-pos")
    local minutes, remainder = divmod(time_pos, 60)
    local hours, minutes = divmod(minutes, 60)
    local seconds = math.floor(remainder)
    local milliseconds = math.floor((remainder - seconds) * 1000)
    local time = string.format("%02d:%02d:%02d.%03d", hours, minutes, seconds, milliseconds)
    mp.osd_message(string.format("Start sample %s", time))
    make_out_file_begin(time,"begin")


end

local function comment_add()
    local handle = io.popen("cutter_desktop_comment_input")
    local result = handle:read("*a")
    handle:close()
    comment_append(result)
end

local function finish_time()
    mp.set_property_native("pause", true)
    comment_add()
    local time_pos = mp.get_property_number("time-pos")
    local minutes, remainder = divmod(time_pos, 60)
    local hours, minutes = divmod(minutes, 60)
    local seconds = math.floor(remainder)
    local milliseconds = math.floor((remainder - seconds) * 1000)
    local time = string.format("%02d:%02d:%02d.%03d", hours, minutes, seconds, milliseconds)
    mp.osd_message(string.format("Ended sample %s", time))
    make_out_file_end(time,"end")
    runned_times=runned_times+1
end



mp.add_key_binding("c", "get_time", get_time)
mp.add_key_binding("e", "finish_time", finish_time)
