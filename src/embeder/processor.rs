pub(in crate) mod cpu {
  use rlua::Lua;
  use std::fs;

  use crate::vars::CONFIG;

  pub(in crate) unsafe fn test() -> Result<String, ()> {
    let cwd = CONFIG.get(&"script".to_string());
    match cwd {
      Ok(path) => {
        let f_path = format!("{cwd}/test.lua", cwd = path);
        match read_file(&f_path) {
          Ok(content) => {
            Lua::new().context(|lua| {
              let globals = lua.globals();
              match lua.load(&content).exec() {
                Ok(_) => {
                  // test the value "res"
                  match globals.get::<_, String>("res") {
                    Ok(res) => Ok(res),
                    Err(err) => {
                      println!("{}", err);
                      Err(())
                    }
                  }
                },
                Err(err) => {
                  println!("\x1b[31mFail to execute the Lua Script at\x1b[0m {path}, \x1b[31mError:\x1b[0m\n{error}", error = err, path = &f_path);
                  Err(())
                }
              }
            })
          },
          Err(_) => {
            println!("\x1b[31mFail to load the Lua Script at\x1b[0m {path}", path = &f_path);
            Err(())
          }
        }
      },
      Err(_) => {
        println!("\x1b[31mFail to load the CWD \x1b[0m");
        Err(())
      }
    }
  }

  /// Read a file
  fn read_file(path: &String) -> Result<String, ()> {
    match fs::read_to_string(path) {
      Ok(data) => Ok(data),
      _ => Err(())
    }
  }
}