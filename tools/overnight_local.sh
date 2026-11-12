#!/bin/bash
# Unattended: wait for vLLM install, start the server in WSL, then fill short skeleton methods locally.
LOG=/c/Users/karti/sumo-rs-local-fill.log
echo "$(date) waiting for vllm install" >> $LOG
until wsl.exe -d Ubuntu-24.04 -e bash -c 'source ~/vllm-env/bin/activate 2>/dev/null && python -c "import vllm" 2>/dev/null' ; do sleep 60; done
echo "$(date) vllm installed; starting server" >> $LOG
wsl.exe -d Ubuntu-24.04 -e bash /mnt/c/Users/karti/sumo-rs/tools/start_vllm.sh > /c/Users/karti/vllm-server.log 2>&1 &   # stays alive as a child of this script
until curl -s http://localhost:8000/v1/models | grep -q '"id"'; do sleep 20; done
echo "$(date) server up; filling" >> $LOG
cd /c/Users/karti/sumo-rs-local
python /c/Users/karti/sumo-rs/tools/local_fill.py --worktree /c/Users/karti/sumo-rs-local --max-lines 15 >> $LOG 2>&1
echo "$(date) local fill finished" >> $LOG
