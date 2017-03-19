extern crate nix;
use nix::sched::*;

use  std::os::unix::io::RawFd;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub fn setup(file_in: RawFd, file_out: RawFd) {
    match unshare(CLONE_FILES | CLONE_FS | CLONE_NEWIPC | CLONE_NEWNET | CLONE_NEWNS |
                  CLONE_NEWPID | CLONE_NEWUSER | CLONE_NEWUTS) {
        Ok(_) => println!("sandheap: set up"),
        Err(e) => println!("sandheap: unshare failed with error {}", e),
    }
}
