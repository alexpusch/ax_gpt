# Ax ChatGPT

Ax Chat GPT directly from the terminal

![Demo](/assets/image.png?raw=true "Optional Title")

### Installation
`cargo install ax`

### Usage example
`ax how to open a file in python`

- Each terminal window has it's own context

### Config
Create or edit the file `$HOME/.config/ax_gpt.json`

```json
{
  "openai_api_key": "OPENAI API KEY",
  "system_prompt": "You are a programmers assistant",
  "model": "gpt-3.5-turbo",
  "sessions_depth": 4
}
```

- `openai_api_key` - get it from [https://platform.openai.com](https://platform.openai.com)
- `system_prompt` - change the personality of the AI
- `model` - change the used models. More details [here](https://platform.openai.com/docs/api-reference/completions/create#completions/create-model) 
- `sessions_depth` - how many past messages should the answer take into account. Note: the larger the context, the more tokens you spend for each request