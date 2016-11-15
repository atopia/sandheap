extern crate nix;
use nix::sched::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn sandheap_setup() {
    match unshare(CLONE_FILES | CLONE_FS | CLONE_NEWIPC | CLONE_NEWNET | CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWUSER | CLONE_NEWUTS) {
        Ok(_) => println!("set up sandheap"),
        Err(e) => println!("unshare failed with error {}", e)
    }
}
