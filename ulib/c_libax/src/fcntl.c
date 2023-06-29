#include <fcntl.h>
#include <libax.h>
#include <stdio.h>

#ifdef AX_CONFIG_FD

int fcntl(int fd, int cmd, ... /* arg */)
{
    unsigned long arg;
    va_list ap;
    va_start(ap, cmd);
    arg = va_arg(ap, unsigned long);
    va_end(ap);

    return ax_fcntl(fd, cmd, arg);
}

#endif // AX_CONFIG_FD

#ifdef AX_CONFIG_FS

int open(const char *filename, int flags, ...)
{
    mode_t mode = 0;

    if ((flags & O_CREAT) || (flags & O_TMPFILE) == O_TMPFILE) {
        va_list ap;
        va_start(ap, flags);
        mode = va_arg(ap, mode_t);
        va_end(ap);
    }

    return ax_open(filename, flags, mode);
}

#endif // AX_CONFIG_FS
