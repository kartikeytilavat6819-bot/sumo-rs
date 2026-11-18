#!/bin/bash
cd "$(dirname "$0")/../.." || exit 1
git rm tools/kaggle/agent_run.sh tools/kaggle/oc29_cleanup.sh
