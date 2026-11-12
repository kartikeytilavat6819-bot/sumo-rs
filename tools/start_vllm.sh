#!/bin/bash
# Start the local vLLM server (WSL) for tools/local_fill.py. 8 GB GPU -> 7B coder model, 4-bit AWQ.
# Usage: start_vllm.sh [max_model_len]   (default 8192; overnight_local.sh retries with 4096)
MAXLEN=${1:-8192}
source ~/vllm-env/bin/activate
# vLLM JIT-compiles some kernels and needs nvcc: use the CUDA toolkit pip installed in the venv
export CUDA_HOME=~/vllm-env/lib/python3.12/site-packages/nvidia/cu13
export PATH="$CUDA_HOME/bin:$PATH" LD_LIBRARY_PATH="$CUDA_HOME/lib:${LD_LIBRARY_PATH:-}"
# FlashInfer JIT kernels fail to compile here (its bundled CCCL headers don't match the pip nvcc):
# use vLLM's built-in sampler and the prebuilt flash-attention backend instead (no compiling needed)
export VLLM_USE_FLASHINFER_SAMPLER=0 VLLM_ATTENTION_BACKEND=FLASH_ATTN
exec vllm serve Qwen/Qwen2.5-Coder-7B-Instruct-AWQ --quantization awq --max-model-len "$MAXLEN" \
  --gpu-memory-utilization 0.80 --max-num-seqs 8 --enforce-eager --port 8000 --host 0.0.0.0
