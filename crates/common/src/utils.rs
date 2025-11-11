use std::ffi::OsString;

pub fn get_log_level<'a>(args: impl Iterator<Item = OsString>) -> &'a str {
  for arg in args {
    if let Some(arg_str) = arg.to_str() {
      if arg_str == "--verbose" {
        return "info";
      } else if let Some(flags) = arg_str.strip_prefix("-") {
        let mut v_count = 0;
        let mut is_v_flag = true;
        for c in flags.chars() {
          if c != 'v' {
            is_v_flag = false;
            break;
          }
          v_count += 1;
        }

        if is_v_flag {
          match v_count {
            1 => return "info",
            2 => return "debug",
            v if v >= 3 => return "trace",
            _ => (),
          }
        }
      }
    }
  }

  "warn"
}
