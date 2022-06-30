// See https://aka.ms/new-console-template for more information
using System.Diagnostics;
using System.Net;
using System.IO.Compression;
using comment_request_windows_installer;
var url_for_bin = "";
var os = Environment.OSVersion;
var install_ffmpeg_using_choco = " gsudo.exe choco install ffmpeg --force  --yes  --accept-license --verbose";
var install_mpv_using_choco = "gsudo.exe choco install mpv --force  --yes  --accept-license --verbose";
var gsudo_download_url = "https://github.com/gerardog/gsudo/releases/download/v1.3.0/gsudo.v1.3.0.zip";
var choco_install = "gsudo.exe powershell.exe Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))";
Console.WriteLine("Installing console part for Windows!");
Console.WriteLine("OS in use: " +os.VersionString+" "+os.ServicePack+" "+os.Platform);
Console.WriteLine("Getting gsudo ");
using (var client = new WebClient())
{
    client.DownloadFile(gsudo_download_url, "tmp.zip");
}

ZipFile.ExtractToDirectory("tmp.zip", "./");
Console.WriteLine("Extracted Successfully");

Console.WriteLine("Getting gsudo ");

CommandRunner cmd = new CommandRunner();
cmd.run_command(choco_install);
cmd.run_command(install_ffmpeg_using_choco);
cmd.run_command(install_mpv_using_choco);




