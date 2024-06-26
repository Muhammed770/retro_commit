use chrono::{Utc, Duration};
use rand::Rng;
use std::fs::File;
use std::io::{self, ErrorKind, Write};
use std::process::{Command, Output};
use std::path::Path;

const FILE_PATH: &str = "data.json";

fn main() {
    if !is_git_initialized() {
        println!("Git is not initialized in this directory. Initializing...");
        initialize_git().expect("Failed to initialize Git repository");
    }

    if !is_remote_configured() {
        println!("Git remote is not configured. Please enter the remote URL:");
        let mut remote_url = String::new();
        io::stdin().read_line(&mut remote_url).expect("failed to read remote URL");
        let remote_url = remote_url.trim();
        set_git_remote(remote_url).expect("Failed to set Git remote");
    }
    let def_user_name =  get_git_config_values("user.name").expect("error");
    let def_user_email = get_git_config_values("user.email").expect("error");
    
    let mut choice = String::new();
    println!("Do you want to change DEFAULT user.name {} and user.email {}? (Y/N)",def_user_name,def_user_email);
    io::stdin().read_line(&mut choice)
        .expect("failed to read choice");
    let mut name = String::new();
    let mut email = String::new();
    if choice == "Y" || choice == "y" {
        println!("Enter username: ");
        io::stdin().read_line(&mut name)
            .expect("failed to read username");
        println!("please enter email id linked to your github:");
        io::stdin().read_line(&mut email)
            .expect("failed to read email");
    } else {
        name = def_user_name;
        email = def_user_email;
    }
    let mut n_commits = String::new();
    println!("please enter number of commits:");
    io::stdin().read_line(&mut n_commits)
        .expect("failed to read number of commits");
    let n_commits_str = n_commits.trim();
    let n_commits: i32;
    match n_commits_str.parse::<i32>() {
        Ok(num) => {
            println!("number of commits {}", num);
            n_commits = num;
        },
        Err(_) => {
            println!("please input an integer number of commits");
            return;
        }
    }
    let mut n_days = String::new();
    println!("please enter number of days:");
    io::stdin().read_line(&mut n_days)
        .expect("unable to read no of days");
    let n_days_str = n_days.trim();
    let n_days: i64;
    match n_days_str.parse::<i64>() {
        Ok(num) => {
            println!("no of days = {}", num);
            n_days = num;
        }
        Err(_) => {
            println!("please enter an integer");
            return;
        }
    }

    println!("username:{} ,email:{} ,n_commits:{}, n_days:{}", name.trim(), email.trim(), n_commits, n_days);
    if !is_branch_main() {
        set_branch_main().expect("failed to set main branch");
    }
    match random_commit(name.trim(), email.trim(), n_commits, n_days) {
        Ok(_) => {
            println!("fn runs successfully")
        } Err(_) => {
            println!("error occured")
        }
    }
}

fn get_git_config_values(key:&str) -> io::Result<String> {
    let output = Command::new("git").args(&["config","--get",key]).output().expect("cmd error: git config error");
    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(value)
    } else {
        eprintln!("Failed to get git config from {}: {}",key,String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other,"Failed to get git config"))
    }
}

fn set_branch_main() -> io::Result<()> {
    let output = Command::new("git").args(&["checkout","-B","main"]).output().expect("cmd error :checkout to main branch error");
    if output.status.success() {
        println!("main branch set success");
        Ok(())
    } else {
        eprintln!("Failed to checkout main branch: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Failed to set branch main"))
    }
}

fn is_git_initialized() -> bool {
    Path::new(".git").exists()
}

fn initialize_git() -> io::Result<()> {
    let output = Command::new("git")
        .arg("init")
        .output()?;
    
    if output.status.success() {
        println!("Initialized empty Git repository");
        Ok(())
    } else {
        eprintln!("Failed to initialize Git repository: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Failed to initialize Git repository"))
    }
}

fn is_branch_main() -> bool {
    let output = Command::new("git").arg("branch").output().expect("failed to execute git branch");
    println!("is_branch_main {}",String::from_utf8_lossy(&output.stdout));
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().any(|line| line.trim() == "* main")
}


fn is_remote_configured() -> bool {
    let output = Command::new("git")
        .args(&["remote", "-v"])
        .output()
        .expect("failed to execute git command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    !stdout.trim().is_empty()
}

fn set_git_remote(remote_url: &str) -> io::Result<()> {
    let output = Command::new("git")
        .args(&["remote", "add", "origin", remote_url])
        .output()?;
    
    if output.status.success() {
        println!("Git remote set to {}", remote_url);
        Ok(())
    } else {
        eprintln!("Failed to set Git remote: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Failed to set Git remote"))
    }
}

fn random_commit(name: &str, email: &str, n_commits: i32, n_days: i64) -> io::Result<()> {
    if n_commits == 0 {
        // Use the below command to push all commits when needed
        Command::new("git").arg("push").output().expect("Failed to push commits");
        println!("done");
        return Ok(());
    }

    let random_day = rand::thread_rng().gen_range(0..n_days);
    let commit_date = Utc::now() - Duration::days(random_day);

    println!("{:?}", commit_date);

    let commit_date_str = commit_date.format("%Y-%m-%d %H:%M:%S").to_string();
    write_commit_date_to_file(FILE_PATH, &commit_date_str)?;

    println!("DATA WRITTEN TO FILE: {} with {} commits remaining", commit_date_str, n_commits);

    let add_output = Command::new("git")
        .args(&["add", FILE_PATH])
        .output();

    match add_output {
        Ok(output) => {
            println!("add_output : {}",output.status)
        },
        Err(e) => {
            println!("add_output error: {}",e)
        }
    }
    

    let commit_output = Command::new("git")
        .args(&["commit", "--date", &commit_date_str, "--author", &format!("{} <{}>", name, email),"-m",&commit_date_str])
        .output();

    match commit_output {
        Ok(output) => {
            println!("commit_output : {}",output.status)
        },
        Err(e) => {
            println!("commit_output error :{}",e)
        }
    }


    random_commit(name, email, n_commits - 1, n_days)
}

fn write_commit_date_to_file(path: &str, commit_date: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{{ \"commitDate\": \"{}\" }}", commit_date)?;
    Ok(())
}
