# Set here the value for the sleep timer
time=3600

while :
do 
	echo "Start crawling."
	../feeds2file
	echo "Stop crawling."
	echo "infinite loop - next run in ${time} seconds [ hit CTRL+C to stop]"
	sleep ${time}
done
