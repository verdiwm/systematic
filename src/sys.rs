/* automatically generated by rust-bindgen 0.71.1 */

pub type __uid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
unsafe extern "C" {
    pub fn sd_pid_get_session(
        pid: pid_t,
        session: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_owner_uid(pid: pid_t, uid: *mut uid_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_unit(pid: pid_t, unit: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_user_unit(
        pid: pid_t,
        unit: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_slice(pid: pid_t, slice: *mut *mut ::core::ffi::c_char)
        -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_user_slice(
        pid: pid_t,
        slice: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_machine_name(
        pid: pid_t,
        machine: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pid_get_cgroup(
        pid: pid_t,
        cgroup: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_session(
        pidfd: ::core::ffi::c_int,
        session: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_owner_uid(pidfd: ::core::ffi::c_int, uid: *mut uid_t)
        -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_unit(
        pidfd: ::core::ffi::c_int,
        unit: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_user_unit(
        pidfd: ::core::ffi::c_int,
        unit: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_slice(
        pidfd: ::core::ffi::c_int,
        slice: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_user_slice(
        pidfd: ::core::ffi::c_int,
        slice: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_machine_name(
        pidfd: ::core::ffi::c_int,
        machine: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_pidfd_get_cgroup(
        pidfd: ::core::ffi::c_int,
        cgroup: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_session(
        fd: ::core::ffi::c_int,
        session: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_owner_uid(fd: ::core::ffi::c_int, uid: *mut uid_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_unit(
        fd: ::core::ffi::c_int,
        unit: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_user_unit(
        fd: ::core::ffi::c_int,
        unit: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_slice(
        fd: ::core::ffi::c_int,
        slice: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_user_slice(
        fd: ::core::ffi::c_int,
        slice: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_machine_name(
        fd: ::core::ffi::c_int,
        machine: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_peer_get_cgroup(
        fd: ::core::ffi::c_int,
        cgroup: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_uid_get_state(uid: uid_t, state: *mut *mut ::core::ffi::c_char)
        -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_uid_get_display(
        uid: uid_t,
        session: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_uid_get_login_time(uid: uid_t, usec: *mut u64) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_uid_is_on_seat(
        uid: uid_t,
        require_active: ::core::ffi::c_int,
        seat: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_uid_get_sessions(
        uid: uid_t,
        require_active: ::core::ffi::c_int,
        sessions: *mut *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_uid_get_seats(
        uid: uid_t,
        require_active: ::core::ffi::c_int,
        seats: *mut *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_is_active(session: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_is_remote(session: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_state(
        session: *const ::core::ffi::c_char,
        state: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_uid(
        session: *const ::core::ffi::c_char,
        uid: *mut uid_t,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_username(
        session: *const ::core::ffi::c_char,
        username: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_seat(
        session: *const ::core::ffi::c_char,
        seat: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_start_time(
        session: *const ::core::ffi::c_char,
        usec: *mut u64,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_service(
        session: *const ::core::ffi::c_char,
        service: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_type(
        session: *const ::core::ffi::c_char,
        type_: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_class(
        session: *const ::core::ffi::c_char,
        clazz: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_desktop(
        session: *const ::core::ffi::c_char,
        desktop: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_display(
        session: *const ::core::ffi::c_char,
        display: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_leader(
        session: *const ::core::ffi::c_char,
        leader: *mut pid_t,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_remote_host(
        session: *const ::core::ffi::c_char,
        remote_host: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_remote_user(
        session: *const ::core::ffi::c_char,
        remote_user: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_tty(
        session: *const ::core::ffi::c_char,
        display: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_session_get_vt(
        session: *const ::core::ffi::c_char,
        vtnr: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_seat_get_active(
        seat: *const ::core::ffi::c_char,
        session: *mut *mut ::core::ffi::c_char,
        uid: *mut uid_t,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_seat_get_sessions(
        seat: *const ::core::ffi::c_char,
        ret_sessions: *mut *mut *mut ::core::ffi::c_char,
        ret_uids: *mut *mut uid_t,
        ret_n_uids: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_seat_can_multi_session(seat: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_seat_can_tty(seat: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_seat_can_graphical(seat: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_machine_get_class(
        machine: *const ::core::ffi::c_char,
        clazz: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_machine_get_ifindices(
        machine: *const ::core::ffi::c_char,
        ret_ifindices: *mut *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_get_seats(seats: *mut *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_get_sessions(sessions: *mut *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_get_uids(users: *mut *mut uid_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_get_machine_names(machines: *mut *mut *mut ::core::ffi::c_char)
        -> ::core::ffi::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sd_login_monitor {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn sd_login_monitor_new(
        category: *const ::core::ffi::c_char,
        ret: *mut *mut sd_login_monitor,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_login_monitor_unref(m: *mut sd_login_monitor) -> *mut sd_login_monitor;
}
unsafe extern "C" {
    pub fn sd_login_monitor_flush(m: *mut sd_login_monitor) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_login_monitor_get_fd(m: *mut sd_login_monitor) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_login_monitor_get_events(m: *mut sd_login_monitor) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sd_login_monitor_get_timeout(
        m: *mut sd_login_monitor,
        timeout_usec: *mut u64,
    ) -> ::core::ffi::c_int;
}
