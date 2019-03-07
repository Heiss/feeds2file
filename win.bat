@echo off
:: Set here the value for the sleep timer
SET time=3600

:while
	echo "Start crawling."
	:: Start program and wait /W to complete and in the same console instance /B
	start "" /W /B feeds2file.exe
	echo "Stop crawling."
	echo "infinite loop - next run in %time% seconds [ hit CTRL+C to stop]"
	timeout /T %time%  > nul
	goto :while
