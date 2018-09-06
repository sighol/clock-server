md output
copy ..\target\release\clock-server.exe output
copy clock-server.kmappx output
nuget pack clock-server.nuspec -BasePath output -Outputdirectory .

xcopy /s /q /y *.nupkg "C:\Dev\nuget-local"
