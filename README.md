<div align="center" style="color: lightgreen;">

# Syncr

is an Experimental local-first synchronization tool designed to efficiently sync
directories between machines on the same network.

</div>

---

The project focuses on minimizing data transfer by using content-defined chunking and
binary state persistence.

## Planned Features

- Synchronization of directories between local machines
- Persistent file state storage using `bincode`
- Automatic discovery of peers on the local network
- Content-defined file chunking using rolling hash
