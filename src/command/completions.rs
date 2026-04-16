use crate::{Lim, command::Run, prelude::*};
use clap::{Args, CommandFactory};
use clap_complete::{Shell, generate};

/// Generate shell completions
#[derive(Args)]
pub struct Completions {
    shell: Shell,
}

impl Run for Completions {
    fn run(self, _config: &crate::Config) -> Result<()> {
        let mut cmd = Lim::command();
        let mut buf = Vec::new();
        generate(self.shell, &mut cmd, "lim", &mut buf);

        match self.shell {
            Shell::Zsh => {
                let script = String::from_utf8(buf)
                    .context("Generated completion script is not valid UTF-8")?;

                let script = script.replace("':path:'", "':path:_lim_event_paths'");

                print!("{}", script);
                print!("{}", ZSH_LIM_EVENT_PATHS_FN);
            }
            _ => {
                std::io::Write::write_all(&mut std::io::stdout(), &buf)
                    .context("Failed to write completions")?;
            }
        }

        Ok(())
    }
}

const ZSH_LIM_EVENT_PATHS_FN: &str = r#"
_lim_event_paths() {
  local -a all_paths
  all_paths=(${(f)"$(lim _paths 2>/dev/null)"})

  local current="${words[$CURRENT]}"
  local prefix=""
  if [[ "$current" == *"."* ]]; then
    prefix="${current%.*}."
  fi

  local -a candidates
  local p rest next
  for p in $all_paths; do
    if [[ "$p" == ${prefix}* ]]; then
      rest="${p#$prefix}"
      next="${rest%%.*}"
      candidates+=("${prefix}${next}")
    fi
  done

  compadd -S "" ${(u)candidates}
}
"#;
