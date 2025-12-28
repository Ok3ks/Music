import json
import wandb
import os
import logging

from flax import nnx
from huggingface_hub import snapshot_download
import jax.numpy as jnp
from orbax import checkpoint as ocp

import jax
import optax
import qwix

from tunix.sft import metrics_logger
from tunix.sft import peft_trainer
from tunix.generate import sampler as sampler_lib
from tunix.generate import tokenizer_adapter as tokenizer_lib
from tunix.sft import utils
from tunix.sft.utils import show_hbm_usage

logger = logging.getLogger()
logger.setLevel(logging.INFO)

if "WANDB_API_KEY" in os.environ and os.environ["WANDB_API_KEY"]:
    wandb.login(key=os.environ["WANDB_API_KEY"])
else:
    print("WANDB_API_KEY not found. Skipping wandb login.")

MODEL_ID = ""
TOKENIZER_PATH = ""
BATCH_SIZE = ""
MAX_TARGET_LENGTH = ""
NUM_TPUS = "" #can i use Jax for Cpu finetuning.
USE_QUANTIZATION = False
RANK = 16
ALPHA = 2.0

MAX_STEPS = 100
EVAL_EVERY_N_STEPS = ""
NUM_EPOCHS = ""

# Checkpoint saving

FULL_CKPT_DIR= "./tmp/content/full_ckpts/"
LORA_CKPT_DIR= "./tmp/content/lora_ckpts/"
PROFILING_DIR= "./tmp/content/profiling/"

ignore_patterns = [
    "*.pth"
]

def load_model(model_id=MODEL_ID):
    
    local_model_path = snapshot_download(repo_id=model_id, ignore_patterns=ignore_patterns)
    EOS_TOKENS = []

    generation_config_path = os.path.join(local_model_path, "generation_config.json")
    if os.path.exists(generation_config_path):
        with open(generation_config_path, "r") as f:
            generation_configs = json.load(f)
        EOS_TOKENS = generation_configs.get("eos_token_id", [])
    
    return local_model_path, EOS_TOKENS

def load_tokenizer():
    tokenizer = tokenizer_lib.Tokenizer(tokenizer_path=TOKENIZER_PATH)
    return tokenizer

# show_hbm_usage()
# if tokenizer.eos_id() not in EOS_TOKENS:
#     EOS_TOKENS.append(tokenizer.eos_id())