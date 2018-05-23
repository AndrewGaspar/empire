use super::{status, universe, Error, handles::*, info::MPI_Info};

use conv::*;
use empire::{Comm, comm::{SpawnCommandInfo, SpawnMultipleResult}};
use std::{slice, ffi::{CStr, OsStr, OsString}, os::raw::{c_char, c_int}};

#[cfg(windows)]
use super::windows::win_string_from_ptr;

unsafe fn slice_from_argv_list<'a>(argv: *const *const c_char) -> &'a [*const c_char] {
    let mut offset = 0;
    while !(*argv.add(offset)).is_null() {
        offset += 1;
    }
    slice::from_raw_parts(argv, offset)
}

#[cfg(windows)]
unsafe fn slice_from_argv_list_w<'a>(argv: *const *const u16) -> &'a [*const u16] {
    let mut offset = 0;
    while !(*argv.add(offset)).is_null() {
        offset += 1;
    }
    slice::from_raw_parts(argv, offset)
}

#[no_mangle]
pub extern "C" fn MPI_Comm_spawn(
    command: Option<&c_char>,
    // technically supposed to be *mut *mut c_char, but we don't modify the input arguments.
    args: Option<&*const c_char>,
    maxprocs: c_int,
    info: MPI_Info,
    root: c_int,
    comm: MPI_Comm,
    intercomm: Option<&mut MPI_Comm>,
    array_of_errcodes: Option<&mut Error>,
) -> Error {
    let command_ptr;
    let args_ptr;

    let (command, args) = {
        let held = unsafe { comm.get() };
        if held.rank() == root.value_as().unwrap() {
            command_ptr = command.expect("command must be specified") as *const c_char;
            args_ptr = args.expect("args must be specified") as *const *const c_char;
            (Some(&command_ptr), Some(&args_ptr))
        } else {
            (None, None)
        }
    };

    MPI_Comm_spawn_multiple(
        1,
        command,
        args,
        Some(&maxprocs),
        Some(&info),
        root,
        comm,
        intercomm,
        array_of_errcodes,
    )
}

fn mpi_comm_spawn_multiple_impl(
    count: usize,
    commands: Option<impl Iterator<Item = OsString>>,
    argvs: Option<impl Iterator<Item = Vec<OsString>>>,
    array_of_maxprocs: Option<&c_int>,
    _array_of_info: Option<&MPI_Info>,
    root: usize,
    comm: &Comm,
    intercomm: Option<&mut MPI_Comm>,
    array_of_errcodes: Option<&mut Error>,
) -> Error {
    let commands = if comm.rank() == root {
        let commands = commands.expect("array_of_commands must be specified");
        let argvs = argvs.expect("array_of_argvs must be specified");

        let array_of_maxprocs =
            array_of_maxprocs.expect("array_of_maxprocs must be specified") as *const c_int;

        let array_of_maxprocs = unsafe { slice::from_raw_parts(array_of_maxprocs, count) };

        let maxprocs = array_of_maxprocs
            .iter()
            .cloned()
            .map(|maxproc| maxproc.value_as().unwrap());

        Some(
            izip!(commands, argvs, maxprocs).map(|(command, args, maxproc)| {
                let mut command = SpawnCommandInfo::new(command, args);
                command.max_procs(maxproc);
                command
            }),
        )
    } else {
        None
    };

    let SpawnMultipleResult { comm, results } = mpitry!(comm.spawn_multiple(commands, root));

    let array_of_errcodes =
        array_of_errcodes.expect("array_of_errcodes must be specified") as *mut Error;

    let array_of_errcodes = unsafe { slice::from_raw_parts_mut(array_of_errcodes, count) };

    for (errcode, result) in izip!(array_of_errcodes, &results) {
        *errcode = status::result_to_mpi_error(result);
    }

    {
        let mut locked = universe().write().unwrap();
        let intercomm = intercomm.expect("intercomm must be specified");
        *intercomm = MPI_Comm::new(CommHandle::UserComm(locked.register_comm(comm)));
    }

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_spawn_multiple(
    count: c_int,
    // technically supposed to be *mut *mut c_char, but we don't modify the input arguments.
    array_of_commands: Option<&*const c_char>,
    // technically supposed to be *mut *mut *mut c_char, but we don't modify the input arguments.
    array_of_argv: Option<&*const *const c_char>,
    array_of_maxprocs: Option<&c_int>,
    _array_of_info: Option<&MPI_Info>,
    root: c_int,
    comm: MPI_Comm,
    intercomm: Option<&mut MPI_Comm>,
    array_of_errcodes: Option<&mut Error>,
) -> Error {
    let count = count.value_as().unwrap();
    let root = root.value_as().unwrap();

    let comm = unsafe { comm.get() };

    let (commands, argvs) = if comm.rank() == root {
        let array_of_commands =
            array_of_commands.expect("array_of_commands must be specified") as *const *const c_char;
        let array_of_argv =
            array_of_argv.expect("array_of_argv must be specified") as *const *const *const c_char;

        let array_of_commands = unsafe { slice::from_raw_parts(array_of_commands, count) };

        let array_of_argv = unsafe { slice::from_raw_parts(array_of_argv, count) };

        let commands = array_of_commands.iter().map(|command| {
            OsStr::new(unsafe { CStr::from_ptr(*command) }.to_str().unwrap()).to_os_string()
        });

        let argvs = array_of_argv.iter().map(|argv| {
            unsafe { slice_from_argv_list(*argv) }
                .iter()
                .map(|arg| {
                    OsStr::new(unsafe { CStr::from_ptr(*arg) }.to_str().unwrap()).to_os_string()
                })
                .collect::<Vec<OsString>>()
        });

        (Some(commands), Some(argvs))
    } else {
        (None, None)
    };

    mpi_comm_spawn_multiple_impl(
        count,
        commands,
        argvs,
        array_of_maxprocs,
        _array_of_info,
        root,
        &*comm,
        intercomm,
        array_of_errcodes,
    )
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn MPI_Comm_spawn_multipleW(
    count: c_int,
    // technically supposed to be *mut *mut c_char, but we don't modify the input arguments.
    array_of_commands: Option<&*const u16>,
    // technically supposed to be *mut *mut *mut c_char, but we don't modify the input arguments.
    array_of_argv: Option<&*const *const u16>,
    array_of_maxprocs: Option<&c_int>,
    _array_of_info: Option<&MPI_Info>,
    root: c_int,
    comm: MPI_Comm,
    intercomm: Option<&mut MPI_Comm>,
    array_of_errcodes: Option<&mut Error>,
) -> Error {
    let count = count.value_as().unwrap();
    let root = root.value_as().unwrap();

    let comm = unsafe { comm.get() };

    let (commands, argvs) = if comm.rank() == root {
        let array_of_commands =
            array_of_commands.expect("array_of_commands must be specified") as *const *const u16;
        let array_of_argv =
            array_of_argv.expect("array_of_argv must be specified") as *const *const *const u16;

        let array_of_commands = unsafe { slice::from_raw_parts(array_of_commands, count) };

        let array_of_argv = unsafe { slice::from_raw_parts(array_of_argv, count) };

        let commands = array_of_commands
            .iter()
            .map(|&command| unsafe { win_string_from_ptr(command) });

        let argvs = array_of_argv.iter().map(|argv| {
            unsafe { slice_from_argv_list_w(*argv) }
                .iter()
                .map(|&arg| unsafe { win_string_from_ptr(arg) })
                .collect::<Vec<_>>()
        });

        (Some(commands), Some(argvs))
    } else {
        (None, None)
    };

    mpi_comm_spawn_multiple_impl(
        count,
        commands,
        argvs,
        array_of_maxprocs,
        _array_of_info,
        root,
        &*comm,
        intercomm,
        array_of_errcodes,
    )
}
