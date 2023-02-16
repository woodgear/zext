use anyhow::{anyhow, Result};
use humantime::format_duration;
use time::{format_description::well_known::Iso8601, PrimitiveDateTime};
use zsh_module::{Builtin, MaybeError, Module, ModuleBuilder, Opts};
zsh_module::export_module!(Zext, setup);

struct Zext;

struct ZextI;

impl ZextI {
    fn time_diff_cmd(&mut self, _name: &str, args: &[&str], _opts: Opts) -> Result<()> {
        if args.len() != 2 {
            return Err(anyhow!("usage time-diff $DATE $DATE").into());
        }
        let start = args
            .get(0)
            .ok_or(anyhow!("could not get start time"))?
            .trim()
            .to_string();
        let end = args
            .get(1)
            .ok_or(anyhow!("could not get end time"))?
            .trim()
            .to_string();

        let start = PrimitiveDateTime::parse(&start, &Iso8601::DEFAULT)?;
        let end = PrimitiveDateTime::parse(&end, &Iso8601::DEFAULT)?;
        println!("{}", format_duration((end - start).unsigned_abs()));
        Ok(())
    }
}

impl Zext {
    fn time_diff_cmd(&mut self, name: &str, args: &[&str], opts: Opts) -> MaybeError {
        ZextI.time_diff_cmd(name, args, opts)?;
        Ok(())
    }
    fn zext_info(&mut self, _name: &str, _args: &[&str], _opts: Opts) -> MaybeError {
        println!("hi,im zext");
        Ok(())
    }
}

fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    let module = ModuleBuilder::new(Zext)
        .builtin(Zext::time_diff_cmd, Builtin::new("time-diff"))
        .builtin(Zext::zext_info, Builtin::new("zext-info"))
        .build();
    Ok(module)
}
