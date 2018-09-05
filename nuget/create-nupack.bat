md output
copy ..\target\release\clock-server.exe output
nuget pack clock-server.nuspec -BasePath output -Version 1.0.0 -Outputdirectory .

xcopy /s /q /y *.nupkg "C:\Dev\nuget-local"

pause