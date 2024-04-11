use sysinfo::Pid;

pub struct Process {
    pub pid: Pid,
    pub name: String,
    pub path: String,
    pub memory: u64,
    pub amount: u16
}