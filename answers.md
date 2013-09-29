Title: Problem Set 2 Answers
Authors: Evan Teague and Josh Lisko

1) I noticed the process "_usbmuxd," and after doing a little research I found out that it is essentially the connection between iTunes and an iDevice. It's something that constantly waits for the device to connect to the computer, and once one does it opens up a TCP connection. The computer and the device undergo a handshake protocol to authenticate a connection. Interesting, I ran "ps aux" multiple times and only found "usbmuxd" once, so one can tell that it checks every so often for an iDevice. 

Source: theiphonewiki.com/wiki/Usbmux

2) When opening Spotify, the CPU usage percentage jumped dramatically, close to 80%, but soon dropped down to ~3.0%. But this was not the most interesting aspect. After it had been open for about 20 seconds, it suddenly spiked to 101%. I was confused about how it could exceed 100, and found out that if you have multiple cores, then you can get number of cores * 100 percentage. So Spotify was using all of one core and a bit of another. 

Source: superuser.com/questions/457624/why-is-the-top-command-showing-a-cpu-usage-of-799

3) I created the piped command "lsof -i -nP | grep IPv4 | sort -nk 2 | IPv4_WebProcesses.txt," which gets all of the web processes that still use IPv4, instead of IPv6, sorts them by process ID, and outputs them to a text file.

Extra features to implement:
> test.txt flush ability
www.commandlinefu.com/commands/browse/sort-by-votes


