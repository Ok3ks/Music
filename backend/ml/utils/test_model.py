from transformers import AutoTokenizer, AutoModelForCausalLM

#Initialise model
#Plug in Weights and Biases
#Test Initial generation before finetuning
#lora finetuning


def model_inference(content:str, model_name="Qwen/Qwen3-0.6B") :

    tokenizer = AutoTokenizer.from_pretrained(model_name, trust_remote_code=True)
    model = AutoModelForCausalLM.from_pretrained(model_name, trust_remote_code=True)
    messages = [
        {"role": "user", "content": f"{content}"},
    ]
    inputs = tokenizer.apply_chat_template(
        messages,
        add_generation_prompt=True,
        tokenize=True,
        return_dict=True,
        return_tensors="pt",
    ).to(model.device)

    outputs = model.generate(**inputs, max_new_tokens=40)
    print(tokenizer.decode(outputs[0][inputs["input_ids"].shape[-1]:]))

if __name__ == "__main__":
    model_inference("what tasks can you efficiently execute , i mean your capabilities and where you shine on a day to day basis. Can you write songs?")