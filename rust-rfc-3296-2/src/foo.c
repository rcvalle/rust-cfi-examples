#include <stdio.h>
#include <stdlib.h>

// This definition has the type id "_ZTSFvlE".
void
hello_from_c(long arg)
{
    printf("Hello from C!\n");
}

// This definition has the type id "_ZTSFvPFvlElE"--this can be ignored for the
// purposes of this example.
void
indirect_call_from_c(void (*fn)(long), long arg)
{
    // This call site tests whether the destination pointer is a member of the
    // group derived from the same type id of the fn declaration, which has the
    // type id "_ZTSFvlE".
    //
    // Notice that since the test is at the call site and generated by Clang,
    // the type id used in the test is encoded by Clang.
    fn(arg);
}