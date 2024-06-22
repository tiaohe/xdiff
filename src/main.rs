use anyhow::{Ok, Result};
use clap::Parser;
use std::io::Write;
use dialoguer::{Input, MultiSelect};
use dialoguer::theme::ColorfulTheme;
use xdiff::{cli::{Action, Args, RunArgs}, DiffConfig, DiffProfile, RequestProfile, ResponseProfile};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.action {
        Action::Run(args) => run(args).await?,
        Action::Parse => parse()?,
        _ => panic!("Not implemented"),
    }

    Ok(())
}
async fn parse() -> Result<()> {
    let theme = ColorfulTheme::default();
    let url1 = Input::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;
    let url2 = Input::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;
    let profile = Input::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;
    let headers = [
        "report-to",
        "date",
        "x-ratelimit-remaining",
        "x-ratelimit-reset",
        "cf-ray",
        "age",
    ];
    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select headers to skip")
        .items(&headers)
        .interact()?;

    let skip_headers = chosen.iter().map(|i| headers[*i]).collect();
    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;
    let res = ResponseProfile::new(skip_headers, vec![]);
    let profile = DiffProfile::new(req1, req2, skip_headers.clone());
    let result = serde_yaml::to_string(&profile)?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    writeln!(stdout, "{}", result)?;

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!("Profile {} not found in config file {}",
        args.profile,
        config_file,)
    })?;
    
    let extra_args = args.extra_params.into();
    let output = profile.diff(extra_args).await?;
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", output)?;

    Ok(())
}
