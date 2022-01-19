use sysinfo::{ProcessExt, System, SystemExt};
use process_memory::{Memory, DataMember, Pid, TryIntoProcessHandle};

mod alchemy;
use alchemy::{init, recipe, print_recipe};


fn main() {
    let s = System::new_all();

    let noita_processes = s.process_by_name("noita.exe");
    let noita_pid: Pid;

    if noita_processes.len() > 1 {
        println!("Multiple Noita Instances found, first one is chosen.");
    }
    noita_pid = noita_processes[0].pid() as Pid;
    println!("Found Noita: {} {} ",
             noita_processes[0].pid(), noita_processes[0].name());

    let handle = noita_pid.try_into_process_handle().unwrap();
    let mut member = DataMember::<u32>::new(handle);

    let seed_offset = 0xFEE850;
    member.set_offset(vec![seed_offset]);

    let seed = member.read().unwrap() as i64;

    let (lc,lc_prob,iseed) = recipe(seed, init(seed));
    let (ap,ap_prob,_x) = recipe(seed, iseed);
    print_recipe(seed, lc, ap, 0,[lc_prob, ap_prob]);
}