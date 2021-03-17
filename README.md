# ssh-ports

Expand a given list of ports into the reverse proxy format that `ssh` expects:

```bash
$ ssh $(sp 5000 8000 10000 10001 7000) <host>

# expands to

$ ssh -L 5000:localhost:5000 -L 8000:localhost:8000 -L 10000:localhost:10000 -L 10001:localhost:10001 -L 7000:localhost:7000 <host>
```