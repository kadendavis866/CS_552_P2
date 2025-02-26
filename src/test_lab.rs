#[cfg(test)]
mod test {
    use crate::test_wrapper_get_prompt as get_prompt;
    use crate::test_wrapper_parse_cmd as parse_cmd;
    use crate::builtin;
    use std::env;

    fn preserve_prompt<T>(test: T) -> ()
    where
        T: FnOnce() -> () + std::panic::UnwindSafe,
    {
        let prompt = env::var_os("MY_PROMPT");
        let result = std::panic::catch_unwind(|| test());
        match prompt {
            Some(val) => env::set_var("MY_PROMPT", val),
            None => env::remove_var("MY_PROMPT"),
        }
        assert!(result.is_ok())
    }

    #[test]
    fn test_get_prompt_default() {
        let exp = "shell> ";
        preserve_prompt(|| {
            env::remove_var("MY_PROMPT");
            let res = get_prompt();
            assert_eq!(res, exp);
        });
    }

    #[test]
    fn test_get_prompt_custom() {
        let prompt = "test>>";
        let exp = "test>> ";
        preserve_prompt(|| {
            env::set_var("MY_PROMPT", prompt);
            let res = get_prompt();
            assert_eq!(res, exp);
        });
    }

    #[test]
    fn test_cmd_parse() {
        let line = "foo -v";
        let (cmd, args) = parse_cmd(line);
        assert_eq!(cmd, "foo");
        assert_eq!(args, vec!["-v"]);
    }

    #[test]
    fn test_cmd_parse_no_args() {
        let line = "foo";
        let (cmd, args) = parse_cmd(line);
        assert_eq!(cmd, "foo");
        assert_eq!(args, Vec::<&str>::new());
    }

    #[test]
    fn test_cmd_parse_empty() {
        let line = "";
        let (cmd, args) = parse_cmd(line);
        assert_eq!(cmd, "");
        assert_eq!(args, Vec::<&str>::new());
    }

    #[test]
    fn test_cmd_parse_whitespace() {
        let line = "  ";
        let (cmd, args) = parse_cmd(line);
        assert_eq!(cmd, "");
        assert_eq!(args, Vec::<&str>::new());
    }

    #[test]
    fn test_cmd_parse_multiple_args() {
        let line = "foo -a -b";
        let (cmd, args) = parse_cmd(line);
        assert_eq!(cmd, "foo");
        assert_eq!(args, vec!["-a", "-b"]);
    }

    #[test]
    fn test_cmd_parse_multiple_args_whitespace() {
        let line = "  foo  -a   -b   ";
        let (cmd, args) = parse_cmd(line);
        assert_eq!(cmd, "foo");
        assert_eq!(args, vec!["-a", "-b"]);
    }

    #[test]
    fn test_pwd() {
        let result = builtin::pwd();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), env::current_dir().unwrap());
    }

    #[test]
    fn test_cd_root() {
        let result = builtin::cd("/");
        assert!(result.is_ok());
        assert_eq!(env::current_dir().unwrap(), std::path::PathBuf::from("/"));
    }

    #[test]
    fn test_cd_nonexistent() {
        let exp = env::current_dir().unwrap();
        let result = builtin::cd("/nonexistent");
        assert!(result.is_err());
        let res = env::current_dir().unwrap();
        assert_eq!(res, exp);
    }

    #[test]
    fn test_cd_dot() {
        let exp = env::current_dir().unwrap();
        let result = builtin::cd(".");
        assert!(result.is_ok());
        let res = env::current_dir().unwrap();
        assert_eq!(res, exp);
    }

    #[test]
    fn test_cd_home() {
        let exp = env::var("HOME").unwrap();
        let result = builtin::cd("~");
        assert!(result.is_ok());
        let res = env::current_dir().unwrap();
        assert_eq!(res, std::path::PathBuf::from(exp));
    }
}
