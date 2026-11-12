#!/bin/bash
source ~/vllm-env/bin/activate
pip install -q --force-reinstall --no-deps --no-cache-dir packaging tokenspeed-triton==3.9.10.post20261028 torch_c_dlpack_ext==0.1.5 triton==3.8.0 2>&1 | tail -3
python -c "import vllm; print('vllm', vllm.__version__)" 2>&1 | tail -1
