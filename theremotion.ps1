Start-Process "theremotion.exe" -ArgumentList "--fullscreen"
Get-WmiObject Win32_process -filter 'name = "theremotion.exe"' | foreach-object { $_.SetPriority(256) }
