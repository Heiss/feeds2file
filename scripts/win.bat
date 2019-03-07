@echo off
:: Set here the value for the sleep timer
SET time=3600

:while
	echo "Start crawling."
	start ../feed2file.exe
	echo "Stop crawling."
	echo "infinite loop - next run in %time% seconds [ hit CTRL+C to stop]"
	timeout /T %time%  > nul
	goto :while
