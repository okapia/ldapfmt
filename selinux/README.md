# ldapfmt, SELinux Policy Module

This directory contains a module that can be used to restrict ldapfmt
to just running those system calls it needs to do it's job.

If it is deployed in a somewhat privileged position such as for sshd's
`AuthorizedKeysCommand` then this significantly limits what the process can do
if compromised.

To build the module, after installing selinux-policy-devel, run:

    make -f /usr/share/selinux/devel/Makefile ldapfmt.pp

Then to install it, run:

    semodule -i ldapfmt.pp

That uses the default priority of 400, it is better to use a lower value from a
packaged rpm or when deploying the policy with a system like ansible by adding,
e.g. `-X300`.

All the policy does is define two types - `ldapfmt_exec_t` and `ldapfmt_t`. The
`.fc` file ensures that the file in `/usr/bin/ldapfmt` has the former of those
types and that file will need editing for different install locations. If `ls
-lZ` shows the wrong type, `restorecon` can be used to set it. The first block
of the `.te` file tells it to transition the process to `ldapfmt_t` when a
binary of type `ldapfmt_exec_t` is run. There's a hardcoded list of source
types, which includes `sshd_t` and common user types like `staff_t`, but you
may need to augment this. The rest of the policy is essentially just the output
of `audit2allow -R` after putting the four relevant types into permissive mode
with, e.g. `semanage permissive -a gssproxy_t`. It's quite possible that one or
two further permissions are needed on a different system or with different
usage.
