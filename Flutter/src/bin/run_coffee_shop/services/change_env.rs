use std::io;
use Flutter::utils::files::replace_line_with_prefix;

pub fn change_env(first_env: &str, second_env: &str, ip: String) -> io::Result<()>{
    replace_line_with_prefix(first_env, "BASE_URL=", &format!("BASE_URL=http://{}:8999/", ip))?;
    replace_line_with_prefix(second_env, "APP_HOST=", &format!("APP_HOST={}", ip))?;
    replace_line_with_prefix(second_env, "APP_URL=", &format!("APP_URL=http://{}:8999", ip))?;

    Ok(())
}