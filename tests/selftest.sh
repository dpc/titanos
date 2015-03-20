export SELFTEST=1
export RELEASE=1

make || exit 1

RES=$(
	( make run | grep -m 1 "SELFTEST END" ) &
	PID=$!
	sleep 3
	kill -9 $PID
)

echo "$RES"
if echo "$RES" | grep "0 ERR" ; then
	exit 0
else
	exit 1
fi
