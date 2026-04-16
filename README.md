# lim

`lim` is a lightweight CLI for personal logs. Record first, query later.

## Motivation

Structured, organized notes have their place in my workflow, but sometimes I just
want to record something quickly and with minimal interuption.

With `lim` I just record a thought and dump it to a log, without stressing about
whether it's in the perfect place.

Later on I (or an LLM) can query it to derive whatever insights I find valuable.

## Logs

`lim` logs are JSONL files where each object represents a single log containing:

- `timestamp`: when the event was logged
- `path`: the hierarchical organization of the event
- `message`: text content
- `attributes`: a flat object containing any other event metadata

Logs can have as much or as little structure as necessary:

```bash
lim add "impact.users_first" "shipped a feature making users' lives 78% better"
lim add "question" "why are things the way that they are?" id=1 category=philosophy
lim add "answer" "google it." id=1
```

## Shell Completions

### zsh

Add the `completions/` directory to your `fpath` before calling `compinit`:

```zsh
# In ~/.zshrc
fpath=(/path/to/lim/completions $fpath)
autoload -Uz compinit && compinit
```

Or copy the file to your completions directory:

```zsh
cp completions/_lim ~/.zsh/completions/_lim
```

To regenerate completions after updating `lim`:

```zsh
lim completions zsh > completions/_lim
```
