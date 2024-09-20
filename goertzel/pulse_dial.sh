N_PULSES=$1
PID=$(pgrep 'goertzel')

for i in $(seq 1 $N_PULSES)
do
    sleep 0.05
    kill -2 $PID
    sleep 0.05
    kill -2 $PID
done
