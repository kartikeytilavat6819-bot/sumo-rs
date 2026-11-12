#!/bin/bash
find ~/vllm-env/lib/python3.12/site-packages/nvidia -name nvcc -type f 2>/dev/null | head -3
