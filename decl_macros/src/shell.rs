macro_rules! shell {
    {$($cmd:expr)=>*} => {
        {
            let mut shell_stuff: Vec<Command> = &[$($cmd),*].into_iter().map(|cmd| std::process::Command::new(cmd[0]).args(cmd[1..]).output()).collect();
            for i in [1..shell_stuff.len()] {
                let in_ = shell_stuff[i].stdin();
                let out_ = shell_stuff[i-1].stdout();

            }
        }

    };
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        let shell_stuff = shell! {
            ["echo", "-e", "1\n2 foo\n3"] => ["grep", "-F", "2"]
        };
        //println!("{:#?}", __shell_cmds);
    }
}
