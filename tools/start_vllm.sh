#!/bin/bash
# Start the local vLLM server (WSL) for tools/local_fill.py. 8 GB GPU -> 7B coder model, 4-bit AWQ.
source ~/vllm-env/bin/activate
exec vllm serve Qwen/Qwen2.5-Coder-7B-Instruct-AWQ --quantization awq --max-model-len 8192 \
  --gpu-memory-utilization 0.90 --max-num-seqs 8 --port 8000 --host 0.0.0.0
